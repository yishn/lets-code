use js_sys::{Function, Reflect};
use tetris::{Direction, Tetris};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue, UnwrapThrowExt};
use wasm_react::{
  c, export_components, h,
  hooks::{use_callback, use_effect, use_js_ref, use_state, Deps},
  props::Style,
  Component,
};
use web_sys::{window, Element, HtmlElement, KeyboardEvent};

mod shape;
mod tetris;

pub struct App {
  width: u32,
  height: u32,
}

impl TryFrom<JsValue> for App {
  type Error = JsValue;

  fn try_from(value: JsValue) -> Result<Self, Self::Error> {
    Ok(App {
      width: Reflect::get(&value, &"width".into())?
        .as_f64()
        .unwrap_or(10.0) as u32,
      height: Reflect::get(&value, &"height".into())?
        .as_f64()
        .unwrap_or(30.0) as u32,
    })
  }
}

impl Component for App {
  fn render(&self) -> wasm_react::VNode {
    let tetris = use_state(|| Tetris::new(self.width, self.height));
    let speed = use_state(|| 500);
    let element_container = use_js_ref::<Element>(None);

    use_effect(
      {
        // Auto focus our container

        let element_container = element_container.clone();

        move || {
          element_container
            .current()
            .and_then(|element| element.dyn_into::<HtmlElement>().ok())
            .map(|element| element.focus().ok());

          || ()
        }
      },
      Deps::none(),
    );

    use_effect(
      {
        let tetris = tetris.clone();
        let speed = *speed.value();

        move || {
          let tick_closure = Closure::new({
            let mut tetris = tetris.clone();

            move || {
              tetris.set(|mut tetris| {
                tetris.tick();
                tetris
              });
            }
          });

          let handle = window()
            .unwrap_throw()
            .set_interval_with_callback_and_timeout_and_arguments_0(
              tick_closure.as_ref().dyn_ref::<Function>().unwrap_throw(),
              speed,
            )
            .unwrap_throw();

          move || {
            drop(tick_closure);
            window().unwrap_throw().clear_interval_with_handle(handle);
          }
        }
      },
      Deps::some(*speed.value()),
    );

    let handle_key_down = use_callback(
      {
        let mut tetris = tetris.clone();
        let mut speed = speed.clone();

        move |evt: KeyboardEvent| {
          let code = evt.code();

          let direction = match &*code {
            "ArrowLeft" => Some(Direction::Left),
            "ArrowRight" => Some(Direction::Right),
            _ => None,
          };

          if let Some(direction) = direction {
            tetris.set(|mut tetris| {
              tetris.shift(direction);
              tetris
            });
          }

          if code == "ArrowUp" {
            tetris.set(|mut tetris| {
              tetris.rotate();
              tetris
            })
          } else if code == "ArrowDown" {
            speed.set(|_| 50);
          }
        }
      },
      Deps::none(),
    );

    let handle_key_up = use_callback(
      {
        let mut speed = speed.clone();

        move |evt: KeyboardEvent| {
          if evt.code() == "ArrowDown" {
            speed.set(|_| 500);
          }
        }
      },
      Deps::none(),
    );

    h!(div)
      .ref_container(&element_container)
      .tabindex(0)
      .on_keydown(&handle_key_down)
      .on_keyup(&handle_key_up)
      .style(
        &Style::new()
          .display("inline-grid")
          .grid_template(format!(
            "repeat({}, 1em) / repeat({}, 1em)",
            self.height, self.width
          ))
          .border("1px solid grey")
          .outline("none"),
      )
      .build(c![
        ..tetris.value().iter_positions().map(|pos| {
          let typ = tetris.value().get(pos);

          h!(div)
            .style(&Style::new().text_indent("-.2em").margin_top("-.2em"))
            .build(c![typ.unwrap_or_default()])
        }),
      ])
  }
}

export_components! { App }
