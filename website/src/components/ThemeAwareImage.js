import React from "react";
import { useColorMode } from "@docusaurus/theme-common";

const ThemeAwareImage = ({ lightSrc, darkSrc, alt, ...props }) => {
  const { colorMode } = useColorMode();

  return (
    <img src={colorMode === "dark" ? darkSrc : lightSrc} alt={alt} {...props} />
  );
};

export default ThemeAwareImage;
