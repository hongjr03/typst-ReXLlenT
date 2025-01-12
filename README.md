# 🦖 ReXLlenT

ReXLlenT is a typst package that helps you convert Excel **xlsx** tables to typst tables, powered by wasm. (Formerly eXMLlent.)

## Quick Start

Start by importing the package:

```typ
#import "@preview/rexllent:0.2.1": xlsx-parser
```

Then you can use `xlsx-parser` function to convert your xlsx Excel table to typst table. Here is an example:

```typ
#xlsx-parser(read("test.xlsx", encoding: none))
```

By passing `sheet-index` parameter, you can specify the sheet index to parse. The default value is 0.

```typ
#xlsx-parser(read("test.xlsx", encoding: none), sheet-index: 1)
```

By toggling parameters below, you can customize the output table:

- `parse-table-style`: Parse table style(columns width, rows height), default is `true`.
- `parse-alignment`: Parse cell content alignment, default is `true`.
- `parse-stroke`: Parse cell stroke, default is `true`.
- `parse-fill`: Parse cell fill, default is `true`.
- `parse-font`: Parse font style, default is `true`.

Extra arguments passed to `xlsx-parser` function will be passed to `table`. Feel free to customize the output table.

Have fun!

## Example

- Excel Table

  ![Excel](assets/excel.png)

- Typst Table (with default parameters)

  ![Typst](assets/example1.png)

- Typst Table (with `parse-table-style: false`)

  ![Typst](assets/example2.png)

- Typst Table (with `parse-alignment: false`)

  ![Typst](assets/example3.png)

- Typst Table (with `parse-stroke: false`)

  ![Typst](assets/example4.png)

- Typst Table (with `parse-fill: false`)

  ![Typst](assets/example5.png)

- Typst Table (with `parse-font: false`)

  ![Typst](assets/example6.png)

## Pixel Art

You can also convert pixel art to typst table using ReXLlenT. Here are some examples:

- Typst Logo
  
  ![typst](assets/typst_example1.png)

- Typst Guy

  ![typst_guy](assets/typst_example2.png)

- *Impression, soleil levant*

  ![monet](assets/typst_example3.png)

## Limitations

ReXLlenT is still in development and PRs are welcome. Here are some limitations:

- Only supports xlsx format.
- Not yet implemented in-cell image parsing.
- Cannot detect special characters in typst, which may cause parsing errors.

## Credits

- [lublak/typst-spreet-package](https://github.com/lublak/typst-spreet-package)
- [MathNya/umya-spreadsheet](https://github.com/MathNya/umya-spreadsheet)

## License

This package is licensed under the MIT License.

## Star History

<a href="https://star-history.com/#hongjr03/typst-rexllent&Date">
 <picture>
   <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=hongjr03/typst-rexllent&type=Date&theme=dark" />
   <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=hongjr03/typst-rexllent&type=Date" />
   <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=hongjr03/typst-rexllent&type=Date" />
 </picture>
</a>
