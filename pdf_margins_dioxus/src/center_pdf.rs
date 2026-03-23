use std::collections::HashMap;
use std::fmt;

use lopdf::{dictionary, Dictionary, Document, Object, ObjectId, Stream};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PaperSize {
    Letter,
    Legal,
    Tabloid,
    Ledger,
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    B4,
    B5,
    FourA0,
    TwoA0,
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    C10,
    Executive,
    Folio,
    GovernmentLetter,
    JuniorLegal,
}

impl PaperSize {
    pub fn dimensions(&self) -> (f64, f64) {
        match self {
            Self::Letter => (612.0, 792.0),
            Self::Legal => (612.0, 1008.0),
            Self::Tabloid => (792.0, 1224.0),
            Self::Ledger => (1224.0, 792.0),
            Self::A0 => (2383.94, 3370.39),
            Self::A1 => (1683.78, 2383.94),
            Self::A2 => (1190.55, 1683.78),
            Self::A3 => (841.89, 1190.55),
            Self::A4 => (595.28, 841.89),
            Self::A5 => (419.53, 595.28),
            Self::A6 => (297.64, 419.53),
            Self::B4 => (708.66, 1000.63),
            Self::B5 => (498.90, 708.66),
            Self::FourA0 => (4767.87, 6740.79),
            Self::TwoA0 => (3370.39, 4767.87),
            Self::C0 => (2599.37, 3676.54),
            Self::C1 => (1836.85, 2599.37),
            Self::C2 => (1298.27, 1836.85),
            Self::C3 => (918.43, 1298.27),
            Self::C4 => (649.13, 918.43),
            Self::C5 => (459.21, 649.13),
            Self::C6 => (323.15, 459.21),
            Self::C7 => (229.61, 323.15),
            Self::C8 => (161.57, 229.61),
            Self::C9 => (113.39, 161.57),
            Self::C10 => (79.37, 113.39),
            Self::Executive => (521.86, 756.0),
            Self::Folio => (612.0, 936.0),
            Self::GovernmentLetter => (576.0, 756.0),
            Self::JuniorLegal => (360.0, 576.0),
        }
    }

    pub const ALL: &[PaperSize] = &[
        Self::Letter,
        Self::Legal,
        Self::Tabloid,
        Self::Ledger,
        Self::A0,
        Self::A1,
        Self::A2,
        Self::A3,
        Self::A4,
        Self::A5,
        Self::A6,
        Self::B4,
        Self::B5,
        Self::FourA0,
        Self::TwoA0,
        Self::C0,
        Self::C1,
        Self::C2,
        Self::C3,
        Self::C4,
        Self::C5,
        Self::C6,
        Self::C7,
        Self::C8,
        Self::C9,
        Self::C10,
        Self::Executive,
        Self::Folio,
        Self::GovernmentLetter,
        Self::JuniorLegal,
    ];
}

impl fmt::Display for PaperSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FourA0 => write!(f, "4A0"),
            Self::TwoA0 => write!(f, "2A0"),
            Self::GovernmentLetter => write!(f, "Government Letter"),
            Self::JuniorLegal => write!(f, "Junior Legal"),
            other => write!(f, "{:?}", other),
        }
    }
}

pub struct CenterOptions {
    pub paper_size: (f64, f64),
    pub draw_alignment: bool,
    pub draw_border: bool,
    pub nudge_x: f64,
    pub nudge_y: f64,
    pub nudge_border_x: f64,
    pub nudge_border_y: f64,
}

pub fn center_pdf(pdf_bytes: &[u8], options: &CenterOptions) -> Result<Vec<u8>, String> {
    let source_doc =
        Document::load_mem(pdf_bytes).map_err(|e| format!("Failed to load PDF: {e}"))?;

    let mut new_doc = Document::with_version("1.7");

    // Manually create Pages and Catalog since with_version doesn't
    let pages_id = new_doc.new_object_id();
    let pages_dict = dictionary! {
        "Type" => "Pages",
        "Kids" => Object::Array(vec![]),
        "Count" => Object::Integer(0),
    };
    new_doc
        .objects
        .insert(pages_id, Object::Dictionary(pages_dict));

    let catalog_id = new_doc.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => Object::Reference(pages_id),
    });
    new_doc.trailer.set("Root", Object::Reference(catalog_id));

    let page_ids: Vec<ObjectId> = source_doc.page_iter().collect();

    for &page_id in &page_ids {
        let page_dict = source_doc
            .get_dictionary(page_id)
            .map_err(|e| format!("Failed to get page dict: {e}"))?;

        let media_box = get_media_box(&source_doc, page_dict)?;
        let src_width = media_box[2] - media_box[0];
        let src_height = media_box[3] - media_box[1];

        // Orientation matching
        let (target_w, target_h) = {
            let (tw, th) = options.paper_size;
            let src_landscape = src_width > src_height;
            let tgt_landscape = tw > th;
            if src_landscape != tgt_landscape {
                (th, tw) // swap to match orientation
            } else {
                (tw, th)
            }
        };

        // Center offsets
        let x_offset = (target_w - src_width) / 2.0 + options.nudge_x;
        let y_offset = (target_h - src_height) / 2.0 + options.nudge_y;

        // Embed source page as Form XObject
        let xobject_id =
            embed_page_as_xobject(&source_doc, &mut new_doc, page_id, &media_box)?;

        // Build content stream that places the XObject
        let mut content_ops = format!(
            "q {x} {y} cm /SourcePage Do Q",
            x = x_offset as f32,
            y = y_offset as f32,
        );

        // Alignment corner drawing
        if options.draw_alignment {
            let is_landscape = target_w > target_h;
            if is_landscape {
                // Top-left L-shape
                content_ops.push_str(&format!(
                    "\n2 w 10 {y1} m 30 {y1} l S\n2 w 10 {y2} m 10 {y1} l S",
                    y1 = (target_h - 10.0) as f32,
                    y2 = (target_h - 30.0) as f32,
                ));
            } else {
                // Bottom-left L-shape
                content_ops.push_str("\n2 w 10 10 m 30 10 l S\n2 w 10 30 m 10 10 l S");
            }
        }

        // Border drawing
        if options.draw_border {
            let bx = x_offset + options.nudge_border_x;
            let by = y_offset + options.nudge_border_y;
            let bw = src_width - options.nudge_border_x * 2.0;
            let bh = src_height - options.nudge_border_y * 2.0;
            content_ops.push_str(&format!(
                "\n2 w {bx} {by} {bw} {bh} re S",
                bx = bx as f32,
                by = by as f32,
                bw = bw as f32,
                bh = bh as f32,
            ));
        }

        let content_stream = Stream::new(Dictionary::new(), content_ops.into_bytes());
        let content_id = new_doc.add_object(content_stream);

        // Build resources dict referencing the XObject
        let mut xobject_dict = Dictionary::new();
        xobject_dict.set("SourcePage", Object::Reference(xobject_id));
        let mut resources_dict = Dictionary::new();
        resources_dict.set("XObject", Object::Dictionary(xobject_dict));

        // Build page dict
        let mut new_page_dict = Dictionary::new();
        new_page_dict.set("Type", "Page");
        new_page_dict.set("Parent", Object::Reference(pages_id));
        new_page_dict.set(
            "MediaBox",
            Object::Array(vec![
                Object::Real(0.0),
                Object::Real(0.0),
                Object::Real(target_w as f32),
                Object::Real(target_h as f32),
            ]),
        );
        new_page_dict.set("Contents", Object::Reference(content_id));
        new_page_dict.set("Resources", Object::Dictionary(resources_dict));

        let new_page_id = new_doc.add_object(Object::Dictionary(new_page_dict));

        // Update pages tree
        let pages_obj = new_doc
            .objects
            .get_mut(&pages_id)
            .ok_or("Pages object not found")?;
        let pages_d = pages_obj
            .as_dict_mut()
            .map_err(|e| format!("Pages not a dict: {e}"))?;
        let kids = pages_d
            .get_mut(b"Kids")
            .map_err(|e| format!("No Kids: {e}"))?
            .as_array_mut()
            .map_err(|e| format!("Kids not array: {e}"))?;
        kids.push(Object::Reference(new_page_id));
        let count = kids.len() as i64;
        pages_d.set("Count", Object::Integer(count));
    }

    // Save to bytes
    let mut output = Vec::new();
    new_doc
        .save_to(&mut output)
        .map_err(|e| format!("Failed to save PDF: {e}"))?;

    Ok(output)
}

/// Read MediaBox from a page dict, walking parent chain if inherited.
fn get_media_box(doc: &Document, page_dict: &Dictionary) -> Result<[f64; 4], String> {
    // Try to read MediaBox from this dict
    if let Ok(mb) = page_dict.get(b"MediaBox") {
        if let Ok(arr) = mb.as_array() {
            return parse_media_box_array(arr);
        }
        // Could be a reference
        if let Ok(id) = mb.as_reference()
            && let Ok(obj) = doc.get_object(id)
            && let Ok(arr) = obj.as_array()
        {
            return parse_media_box_array(arr);
        }
    }

    // Walk up via Parent
    if let Ok(parent_ref) = page_dict.get(b"Parent")
        && let Ok(parent_id) = parent_ref.as_reference()
        && let Ok(parent_dict) = doc.get_dictionary(parent_id)
    {
        return get_media_box(doc, parent_dict);
    }

    Err("MediaBox not found".to_string())
}

fn parse_media_box_array(arr: &[Object]) -> Result<[f64; 4], String> {
    if arr.len() < 4 {
        return Err("MediaBox has fewer than 4 elements".to_string());
    }
    let mut vals = [0.0f64; 4];
    for (i, obj) in arr.iter().take(4).enumerate() {
        vals[i] = obj_to_f64(obj)?;
    }
    Ok(vals)
}

fn obj_to_f64(obj: &Object) -> Result<f64, String> {
    match obj {
        Object::Real(f) => Ok(*f as f64),
        Object::Integer(i) => Ok(*i as f64),
        _ => Err(format!("Expected number, got {:?}", obj.enum_variant())),
    }
}

/// Get the content bytes of a page (decompressed, concatenated).
fn get_page_content_bytes(doc: &Document, page_dict: &Dictionary) -> Result<Vec<u8>, String> {
    let contents = match page_dict.get(b"Contents") {
        Ok(c) => c,
        Err(_) => return Ok(vec![]), // no content
    };

    match contents {
        Object::Reference(id) => {
            let obj = doc
                .get_object(*id)
                .map_err(|e| format!("Content ref: {e}"))?;
            match obj {
                Object::Stream(stream) => stream
                    .get_plain_content()
                    .map_err(|e| format!("Decompress: {e}")),
                Object::Array(arr) => collect_stream_array(doc, arr),
                _ => Err("Contents ref points to unexpected type".to_string()),
            }
        }
        Object::Array(arr) => collect_stream_array(doc, arr),
        Object::Stream(stream) => stream
            .get_plain_content()
            .map_err(|e| format!("Decompress: {e}")),
        _ => Err("Unexpected Contents type".to_string()),
    }
}

fn collect_stream_array(doc: &Document, arr: &[Object]) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    for item in arr {
        let id = item
            .as_reference()
            .map_err(|e| format!("Array item not ref: {e}"))?;
        let obj = doc.get_object(id).map_err(|e| format!("Get obj: {e}"))?;
        let stream = obj.as_stream().map_err(|e| format!("Not stream: {e}"))?;
        let data = stream
            .get_plain_content()
            .map_err(|e| format!("Decompress: {e}"))?;
        bytes.extend_from_slice(&data);
    }
    Ok(bytes)
}

/// Deep-copy an object from source to dest document, remapping all object IDs.
fn collect_and_remap_objects(
    source: &Document,
    dest: &mut Document,
    obj: &Object,
    remap: &mut HashMap<ObjectId, ObjectId>,
) -> Object {
    match obj {
        Object::Reference(id) => {
            if let Some(&new_id) = remap.get(id) {
                Object::Reference(new_id)
            } else {
                // Copy referenced object
                if let Ok(source_obj) = source.get_object(*id) {
                    let source_obj_clone = source_obj.clone();
                    // Reserve a new ID first to handle cycles
                    let new_id = dest.new_object_id();
                    remap.insert(*id, new_id);
                    let remapped = collect_and_remap_objects(source, dest, &source_obj_clone, remap);
                    dest.objects.insert(new_id, remapped);
                    Object::Reference(new_id)
                } else {
                    Object::Null
                }
            }
        }
        Object::Array(arr) => {
            let new_arr: Vec<Object> = arr
                .iter()
                .map(|item| collect_and_remap_objects(source, dest, item, remap))
                .collect();
            Object::Array(new_arr)
        }
        Object::Dictionary(dict) => {
            let mut new_dict = Dictionary::new();
            for (key, val) in dict.iter() {
                let new_val = collect_and_remap_objects(source, dest, val, remap);
                new_dict.set(key.clone(), new_val);
            }
            Object::Dictionary(new_dict)
        }
        Object::Stream(stream) => {
            let mut new_dict = Dictionary::new();
            for (key, val) in stream.dict.iter() {
                let new_val = collect_and_remap_objects(source, dest, val, remap);
                new_dict.set(key.clone(), new_val);
            }
            Object::Stream(Stream::new(new_dict, stream.content.clone()))
        }
        // Primitives: clone as-is
        other => other.clone(),
    }
}

/// Embed a source page as a Form XObject in the destination document.
fn embed_page_as_xobject(
    source_doc: &Document,
    new_doc: &mut Document,
    page_id: ObjectId,
    media_box: &[f64; 4],
) -> Result<ObjectId, String> {
    let page_dict = source_doc
        .get_dictionary(page_id)
        .map_err(|e| format!("Get page: {e}"))?;

    // Get content bytes
    let content_bytes = get_page_content_bytes(source_doc, page_dict)?;

    // Get resources and deep-copy to new doc
    let mut remap: HashMap<ObjectId, ObjectId> = HashMap::new();
    let resources_obj = match page_dict.get(b"Resources") {
        Ok(res) => {
            let resolved = match res {
                Object::Reference(id) => source_doc
                    .get_object(*id)
                    .map_err(|e| format!("Resolve resources: {e}"))?,
                other => other,
            };
            collect_and_remap_objects(source_doc, new_doc, resolved, &mut remap)
        }
        Err(_) => {
            // Try parent chain for inherited Resources
            if let Some(res) = find_inherited_resources(source_doc, page_dict) {
                collect_and_remap_objects(source_doc, new_doc, &res, &mut remap)
            } else {
                Object::Dictionary(Dictionary::new())
            }
        }
    };

    // Build Form XObject
    let bbox = vec![
        Object::Real(media_box[0] as f32),
        Object::Real(media_box[1] as f32),
        Object::Real(media_box[2] as f32),
        Object::Real(media_box[3] as f32),
    ];

    let mut xobject_dict = Dictionary::new();
    xobject_dict.set("Type", "XObject");
    xobject_dict.set("Subtype", "Form");
    xobject_dict.set("BBox", Object::Array(bbox));
    xobject_dict.set("Resources", resources_obj);

    let xobject_stream = Stream::new(xobject_dict, content_bytes);
    let xobject_id = new_doc.add_object(xobject_stream);

    Ok(xobject_id)
}

/// Walk the parent chain to find inherited Resources.
fn find_inherited_resources(doc: &Document, page_dict: &Dictionary) -> Option<Object> {
    if let Ok(parent_ref) = page_dict.get(b"Parent")
        && let Ok(parent_id) = parent_ref.as_reference()
        && let Ok(parent_dict) = doc.get_dictionary(parent_id)
    {
        if let Ok(res) = parent_dict.get(b"Resources") {
            return Some(res.clone());
        }
        return find_inherited_resources(doc, parent_dict);
    }
    None
}
