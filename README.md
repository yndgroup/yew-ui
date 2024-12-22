# yew-ui

A collection of UI components for `yew`.

## List of tools used
- [rust](https://www.rust-lang.org) 
- [yew](https://yew.rs/)
- [gloo](https://docs.rs/gloo/latest/gloo/)
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).
- [trunk](https://trunkrs.dev/)
- [tailwindcss](https://tailwindcss.com/)

## Run

- install trunk
```
cargo install trunk --locked
```
- install wasm32-unknown-unknown
```
rustup target add wasm32-unknown-unknown
```

- run examples

```
cd examples
trunk serve
```

## Target component development

> Explanation: The hook has already been implemented, while the fork has not yet been implemented. Welcome everyone to join, everyone gathers firewood and the flames are high

| Name                     | style | event | complete |
| ------------------------ | ----- | ----- | -------- |
| Button                   | ✅     | ✅     | ❌        |
| Icon                     | ✅     | ❌     | ❌        |
| Border                   | ❌     | ❌     | ❌        |
| Container                | ✅     | ❌     | ❌        |
| Layout                   | ❌     | ❌     | ❌        |
| Link                     | ❌     | ❌     | ❌        |
| Text                     | ❌     | ❌     | ❌        |
| Scrollbar                | ❌     | ❌     | ❌        |
| Space                    | ❌     | ❌     | ❌        |
| Typography               | ❌     | ❌     | ❌        |
| Config Provider          | ❌     | ❌     | ❌        |
| Autocomplete             | ❌     | ❌     | ❌        |
| Cascader                 | ❌     | ❌     | ❌        |
| Checkbox                 | ❌     | ❌     | ❌        |
| ColorPicker              | ❌     | ❌     | ❌        |
| DatePicker               | ❌     | ❌     | ❌        |
| DateTimePicker           | ❌     | ❌     | ❌        |
| Form                     | ❌     | ❌     | ❌        |
| Input                    | ❌     | ❌     | ❌        |
| Input Number             | ❌     | ❌     | ❌        |
| InputTag                 | ❌     | ❌     | ❌        |
| Mention                  | ❌     | ❌     | ❌        |
| Radio                    | ❌     | ❌     | ❌        |
| Rate                     | ❌     | ❌     | ❌        |
| Select                   | ❌     | ❌     | ❌        |
| Virtualized Select       | ❌     | ❌     | ❌        |
| Slider                   | ❌     | ❌     | ❌        |
| Switch                   | ❌     | ❌     | ❌        |
| TimePicker               | ❌     | ❌     | ❌        |
| TimeSelect               | ❌     | ❌     | ❌        |
| Transfer                 | ❌     | ❌     | ❌        |
| TreeSelect               | ❌     | ❌     | ❌        |
| Upload                   | ❌     | ❌     | ❌        |
| Avatar                   | ❌     | ❌     | ❌        |
| Badge                    | ❌     | ❌     | ❌        |
| Calendar                 | ❌     | ❌     | ❌        |
| Card                     | ❌     | ❌     | ❌        |
| Carousel                 | ❌     | ❌     | ❌        |
| Collapse                 | ❌     | ❌     | ❌        |
| Descriptions             | ❌     | ❌     | ❌        |
| Empty                    | ❌     | ❌     | ❌        |
| Image                    | ❌     | ❌     | ❌        |
| Infinite Scroll          | ❌     | ❌     | ❌        |
| Pagination               | ❌     | ❌     | ❌        |
| Progress                 | ❌     | ❌     | ❌        |
| Result                   | ❌     | ❌     | ❌        |
| Skeleton                 | ❌     | ❌     | ❌        |
| Table                    | ❌     | ❌     | ❌        |
| Virtualized Table        | ❌     | ❌     | ❌        |
| Tag                      | ❌     | ❌     | ❌        |
| Timeline                 | ❌     | ❌     | ❌        |
| Tour                     | ❌     | ❌     | ❌        |
| Tree                     | ❌     | ❌     | ❌        |
| Tree V2 virtualized tree | ❌     | ❌     | ❌        |
| Statistic                | ❌     | ❌     | ❌        |
| Segmented                | ❌     | ❌     | ❌        |
| Affix                    | ❌     | ❌     | ❌        |
| Anchor                   | ❌     | ❌     | ❌        |
| Backtop                  | ❌     | ❌     | ❌        |
| Breadcrumb               | ❌     | ❌     | ❌        |
| Dropdown                 | ❌     | ❌     | ❌        |
| Menu                     | ❌     | ❌     | ❌        |
| Page Header              | ❌     | ❌     | ❌        |
| Steps                    | ❌     | ❌     | ❌        |
| Tabs                     | ❌     | ❌     | ❌        |
| Alert                    | ❌     | ❌     | ❌        |
| Dialog                   | ❌     | ❌     | ❌        |
| Drawer                   | ❌     | ❌     | ❌        |
| Loading                  | ❌     | ❌     | ❌        |
| Message                  | ❌     | ❌     | ❌        |
| Message Box              | ❌     | ❌     | ❌        |
| Notification             | ❌     | ❌     | ❌        |
| Popconfirm               | ❌     | ❌     | ❌        |
| Popover                  | ❌     | ❌     | ❌        |
| Tooltip                  | ❌     | ❌     | ❌        |
| Divider                  | ❌     | ❌     | ❌        |
| Watermark                | ❌     | ❌     | ❌        |
|                          |       |       |          |
|                          |       |       |          |

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE.txt) file for details.
