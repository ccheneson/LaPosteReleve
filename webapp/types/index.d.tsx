import React from "react";

declare global {
  namespace JSX {
    interface HTMLAttributes {
      styleName?: string;
    }

    interface SVGAttributes {
      styleName?: string;
    }
  }
}
