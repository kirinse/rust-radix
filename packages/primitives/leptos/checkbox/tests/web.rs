use leptos::*;
use radix_leptos_checkbox::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn render<F, N>(f: F)
where
    F: FnOnce() -> N + 'static,
    N: IntoView,
{
    document()
        .body()
        .expect("Document should have body.")
        .replace_children_with_node_0();

    mount_to_body(f)
}

// fn get_by_role(role: String) -> Result<Element, Box<dyn Error>> {
//     let list = document()
//         .query_selector_all(&format!("[role=\"{}\"]", role))
//         .expect("Document should be queried.");
//     match list.length() {
//         0 => Err(format!("No elements found for role \"{}\".", role).into()),
//         1 => Ok(list.get(0).expect("Single node should exist.")),
//         _ => Err(format!("Multiple elements found for role \"{}\".", role).into()),
//     }
// }

const CHECKBOX_ROLE: &str = "checkbox";
const INDICATOR_TEST_ID: &str = "checkbox-indicator";

#[wasm_bindgen_test]
async fn test() {
    render(|| {
        view! {
            <CheckboxTest />
        }
    });
}

#[component]
fn CheckboxTest() -> impl IntoView {
    view! {
        <div>
            <Checkbox attr:aria-label="basic checkbox">
                <CheckboxIndicator attr:data-testid=INDICATOR_TEST_ID />
            </Checkbox>
        </div>
    }
}
