#!/usr/bin/env deno --allow-env --allow-read

import * as path from "jsr:@std/path";

const determineSaveDir = () =>
  Deno.args?.[0] ||
  path.join(
    Deno.env.get("HOME"),
    Deno.build.os === "darwin"
      ? "Library/Application Support/CrossOver/Bottles/Steam/drive_c/users/crossover"
      : "",
    Deno.build.os === "linux"
      ? ".local/share/Steam/steamapps/compatdata/3164500/pfx/drive_c/users/steamuser"
      : "",
    "AppData/LocalLow/TVGS/Schedule I/Saves",
    // TODO: enumerate save directories and pick one
    "76561198071226707/SaveGame_1",
  );

const loadProducts = (saveDir) => {
  const products_json = JSON.parse(
    Deno.readTextFileSync(path.join(saveDir, "Products/products.json")),
  );
  const products = products_json.DiscoveredProducts.reduce((acc, product) => {
    acc[product] = {};
    return acc;
  }, {});
  products_json.MixRecipes.forEach((recipe) => {
    // the Product and Mixer seem inconsistently assigned
    if (!products[recipe.Product] && products[recipe.Mixer]) {
      products[recipe.Mixer][recipe.Product] = recipe.Output;
    } else if (products[recipe.Product] && !products[recipe.Mixer]) {
      products[recipe.Product][recipe.Mixer] = recipe.Output;
    } else {
      console.error(`Product ${recipe.Product}/${recipe.Mixer} not found`);
    }
  });
  for (let [k, v] of Object.entries(products)) {
    if (Object.keys(v).length === 0) {
      // means end of chain
      products[k] = "";
    } else {
      // remove circular references
      for (let [k1, v1] of Object.entries(v)) {
        if (v1 === k) {
          console.log(`Removing circular reference ${k} -> ${k1}`);
          delete products[k][k1];
        }
      }
    }
  }
  return products;
};

const collapseProducts = (products, keys) => {
  rootLoop: for (let i = keys.length - 1; i >= 0; i--) {
    let target = keys[i];
    for (let [k0, v0] of Object.entries(products)) {
      if (k0 !== target) {
        for (let [k1, v1] of Object.entries(products[k0])) {
          if (v1 === target) {
            products[k0][k1] = {
              [target]: products[target],
            };
            delete products[target];
            continue rootLoop;
          }
        }
      }
    }
  }
  return products;
};

export const isCollapsable = (v) => {
  if (typeof v === "object") {
    // check if each value is collapsable
    for (const key in v) {
      if (!isCollapsable(v[key])) {
        return false;
      }
    }
  } else if (typeof v === "string" && v !== "") {
    return false;
  }
  return true;
};

const saveDir = determineSaveDir();
let products = loadProducts(saveDir);
while (Object.keys(products).length > 9) {
  let keys = Object.entries(products).reduce((acc, [k, v]) => {
    if (isCollapsable(v)) {
      acc.push(k);
    }
    return acc;
  }, []);
  products = collapseProducts(products, keys);
}
console.log(JSON.stringify(products, null, 2));
console.log(Object.keys(products).length);
