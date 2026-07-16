import Color from "colorjs.io";
import { generateRadixColors } from "./generate-radix-colors.mts";

const hex = (css: string) => new Color(css).to("srgb").toString({ format: "hex" });
const okl = (h: string) => {
  const c = new Color(h).to("oklch");
  return `L${(c.coords[0] * 100).toFixed(1)} C${(c.coords[1] ?? 0).toFixed(3)} H${(c.coords[2] ?? 0).toFixed(0)}`;
};

// Web surfaces (meridian-online/web app/global.css)
const surfaces = {
  lightBg: hex("oklch(98.5% 0.002 80)"),
  lightCard: hex("oklch(99% 0.001 80)"),
  darkBg: hex("oklch(15.5% 0.004 60)"),
  darkCard: hex("oklch(19.5% 0.004 60)"),
};
console.log("SURFACES");
for (const [k, v] of Object.entries(surfaces)) console.log(`  ${k}: ${v}  (${okl(v)})`);

// Seeds. Gray = true hue-70 warm cream neutral (Radix sand measures ~85-107, too green).
// Semantic seeds picked in OKLCH with moderate chroma, warm-leaning where the role allows.
const seeds: Record<string, string> = {
  maritime: "#4b7a9b", // brand accent, web decision 0010
  gray: hex("oklch(50% 0.008 70)"),
  red: hex("oklch(55% 0.19 27)"), // error
  amber: hex("oklch(72% 0.15 75)"), // warning — warm, hue near the neutrals
  green: hex("oklch(56% 0.14 150)"), // success
  blue: hex("oklch(55% 0.12 240)"), // info — Maritime-adjacent hue family
};
console.log("\nSEEDS");
for (const [k, v] of Object.entries(seeds)) console.log(`  ${k}: ${v}  (${okl(v)})`);

for (const appearance of ["light", "dark"] as const) {
  const background = appearance === "light" ? surfaces.lightBg : surfaces.darkBg;
  console.log(`\n=== ${appearance.toUpperCase()} (bg ${background}) ===`);
  let grayPrinted = false;
  for (const [name, seed] of Object.entries(seeds)) {
    if (name === "gray") continue;
    const r = generateRadixColors({ appearance, accent: seed, gray: seeds.gray, background });
    if (!grayPrinted) {
      console.log(`gray     ${r.grayScale.join(" ")}`);
      grayPrinted = true;
    }
    console.log(`${name.padEnd(8)} ${r.accentScale.join(" ")}`);
  }
}
