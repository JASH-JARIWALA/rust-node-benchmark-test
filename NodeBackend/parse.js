import csv from "csv-parser";
import { createReadStream, writeFile } from "fs";
const node = [];
const rust = [];

createReadStream("node.csv")
  .pipe(csv({ separator: " " }))
  .on("data", (data) => node.push(data))
  .on("end", () => {
    const json = JSON.stringify(node);
    writeFile("./node.json", json, (err) => {
      if (err) {
        console.error(err);
      }
    });
  });
createReadStream("rust.csv")
  .pipe(csv({ separator: " " }))
  .on("data", (data) => rust.push(data))
  .on("end", () => {
    const json = JSON.stringify(rust);
    writeFile("./rust.json", json, (err) => {
      if (err) {
        console.error(err);
      }
    });
  });
