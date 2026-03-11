import { PageSizes, PDFDocument } from 'pdf-lib';

export const centerPdf = async (pdfBytes: string | Uint8Array | ArrayBuffer, options: {
  drawAlignment?: boolean,
  drawBorder?: boolean,
  nudgeHeight?: number,
  nudgeWidth?: number,
  nudgeBorderWidth?: number,
  nudgeBorderHeight?: number,
  paperSize?: [number, number],
}): Promise<Uint8Array> => {
  const drawAlignment = options?.drawAlignment !== false;
  const drawBorder = options?.drawBorder !== false;
  const nudgeHeight = options?.nudgeHeight || 0;
  const nudgeWidth = options?.nudgeWidth || 0;
  const nudgeBorderWidth = options?.nudgeBorderWidth || 0;
  const nudgeBorderHeight = options?.nudgeBorderWidth || 0;
  const paperSize = options?.paperSize?.length === 2 ? options.paperSize : PageSizes.Letter;
  const oldPdfDoc = await PDFDocument.load(pdfBytes);
  const newPdfDoc = await PDFDocument.create();

  for (const oldPage of oldPdfDoc.getPages()) {
    const bothLandscapeOrPortrait = oldPage.getWidth() > oldPage.getHeight() && paperSize[0] > paperSize[1];
    const newPage = newPdfDoc.addPage(bothLandscapeOrPortrait ? paperSize : [paperSize[1], paperSize[0]]);

    const widthOffset = (newPage.getWidth() - oldPage.getWidth()) / 2;
    const heightOffset = (newPage.getHeight() - oldPage.getHeight()) / 2;

    const embedded = await newPdfDoc.embedPage(oldPage);
    newPage.drawPage(embedded, {
      x: widthOffset + nudgeWidth,
      y: heightOffset + nudgeHeight,
    });

    if (drawAlignment) {
      const isLandscape = newPage.getWidth() > newPage.getHeight();
      newPage.drawLine({
        start: { x: 10, y: isLandscape ? newPage.getHeight() - 10 : 10 },
        end: { x: 30, y: isLandscape ? newPage.getHeight() - 10 : 10 },
        thickness: 2,
      });

      newPage.drawLine({
        start: { x: 10, y: isLandscape ? newPage.getHeight() - 30 : 30 },
        end: { x: 10, y: isLandscape ? newPage.getHeight() - 10 : 10 },
        thickness: 2,
      });
    }

    if (drawBorder) {
      newPage.drawRectangle({
        x: widthOffset + nudgeBorderWidth + nudgeWidth,
        y: heightOffset + nudgeBorderHeight + nudgeHeight,
        height: oldPage.getHeight() - (nudgeBorderHeight * 2),
        width: oldPage.getWidth() - (nudgeBorderWidth * 2),
        opacity: 0,
        borderOpacity: 1,
        borderWidth: 2,
      });
    }
  }

  return await newPdfDoc.save();
};
