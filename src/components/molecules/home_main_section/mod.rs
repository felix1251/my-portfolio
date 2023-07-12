use yew::prelude::*;

#[function_component(HomeMainSection)]
pub fn home_main_section() -> Html {
    html! {
        <section class="mx-auto max-w-[85rem] px-6 md:px-8 grid grid-cols-1 md:grid-cols-5 h-[700px] overflow-hidden">
            <div class="md:col-span-3 flex items-center">
                <div class="max-w-[600px] md:mr-14 space-y-5">
                    <h1 class="font-medium text-5xl md:text-6xl transition-colors duration-200 text-secondary dark:text-white">
                        {"I'm a "}
                        <span class="text-primary">{"Fullstack Developer"}</span>
                        {" designing and building products."}
                    </h1>
                    <p class="text-tertiary font-medium">
                        {"Now solving problems in the Information Technology scene."}
                    </p>
                </div>
            </div>
            <div class="col-span-2 hidden md:block bg-tertiary"></div>
        </section>
    }
}
