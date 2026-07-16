// Emit Rust const blocks for meridian-design from the validated Phase 1 values.
const S = {
  gray_light: "f8f7f7 f4f3f2 ebeae9 e4e2e0 dcdad8 d4d2cf cac6c3 b7b3ae 8a847f 7f7a75 605c58 231f1c",
  gray_dark: "0d0c0c 191817 232221 2a2928 32302f 3c3a38 4a4744 64605c 716c67 7e7a75 b7b2ae efeeec",
  maritime_light: "f6f8fa eff3f7 e3eef6 d6e6f2 c7dceb b5cee2 9dbdd5 7ca6c5 4b7a9b 3e6d8d 3b698a 283e4d",
  maritime_dark: "080d12 12191e 172835 1b3547 244256 2f5068 3c627d 4a799a 4b7a9b 466c87 8fc1e4 cde8fc",
  red_light: "faf7f7 f9f2f1 f8e6e3 fed6d1 fbc8c1 f4b8b0 eba49b e18a80 c9302d b91c1e c42a28 621c18",
  red_dark: "130a09 21110f 410a09 590001 6c0004 7f0f11 9a211f c62d2b c9302d b91c1e ff9083 ffcec6",
  amber_light: "f9f8f6 fff3e3 ffe9ba ffdd9a ffd17a ffc467 efb256 de9917 da950b ce8a00 9e6300 4b3718",
  amber_dark: "0f0c08 1b1711 2b2011 3a2709 47300a 563c13 6a4c1d 886122 da950b ce8a00 ebb15b f9e1c1",
  green_light: "f5f9f6 eff5f0 e1f1e3 d1ebd5 bfe3c5 a7d7b0 88c895 58b36f 218a45 057c38 007a35 1e3c25",
  green_dark: "070f08 111a12 142b19 0c3d1c 0f4b23 125b2b 136c33 0e7f3b 218a45 057c38 6fd087 aaf7ba",
  blue_light: "f4f8fb edf4f9 ddeffc cde7fb baddf7 a5d1f0 8ac0e6 5da9dc 1479b0 006ba1 006ca2 183e56",
  blue_dark: "040e16 0b1a25 04293f 003755 004568 0a547c 146694 177bb2 1479b0 1c6b99 78c3f7 c9edff",
};
const V = {
  categorical_light: "0083c4 cca124 31aa8c c13c3b 6a55b8 dd7b2b bf62ad 47944c",
  categorical_dark: "0b89c8 b68f1b 31aa8c c74b47 7b69c6 b45c03 b55fa4 47944c",
  sequential_meridian: "c6e4fb a6d7fa 87c8f6 69baf0 4daae6 359bd9 238cc7 1d7cb2 216d9b 285e81 2d4f67 274154 1b3546",
  diverging_blue_arm: "005389 0070a9 478ebc 79abcf a8c9e2", // pole → lightest
  diverging_red_arm: "e3b8b4 d0938d bb6d67 a54743 8d1a1e", // lightest → pole
};
const rgba = (h: string) =>
  `Rgba::from_u8(0x${h.slice(0, 2)}, 0x${h.slice(2, 4)}, 0x${h.slice(4, 6)}, 0xff)`;
const arr = (name: string, hexes: string, ty: string) => {
  const items = hexes.split(" ").map((h) => `    ${rgba(h)}, // #${h}`);
  return `pub const ${name}: ${ty} = [\n${items.join("\n")}\n];`;
};
console.log("// scales");
for (const [k, v] of Object.entries(S)) console.log(arr(k.toUpperCase(), v, "[Rgba; 12]") + "\n");
console.log("// viz");
console.log(arr("CATEGORICAL_LIGHT", V.categorical_light, "[Rgba; 8]") + "\n");
console.log(arr("CATEGORICAL_DARK", V.categorical_dark, "[Rgba; 8]") + "\n");
console.log(arr("SEQUENTIAL_MERIDIAN", V.sequential_meridian, "[Rgba; 13]") + "\n");
console.log(arr("DIVERGING_BLUE_ARM", V.diverging_blue_arm, "[Rgba; 5]") + "\n");
console.log(arr("DIVERGING_RED_ARM", V.diverging_red_arm, "[Rgba; 5]") + "\n");
