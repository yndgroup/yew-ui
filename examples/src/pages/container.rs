use yew::prelude::*;
use yew_ui::components::prelude::*;
use yew_ui::gloo::console::log;

use yew_ui::toolkit::math::add;

use crate::components::Footer as CFooter;

#[function_component(PageContainer)]
pub fn page_container() -> Html {
    log!(add(1, 2));
    html! {
       <div>
            <dl>
                <dt class="text-4xl pb-4">{"Style 1"}</dt>
                <dd>
                    <Container>
                        <Header class="bg-red-200 h-[60px]">
                            {"header"}
                        </Header>
                        <Main class="bg-blue-200 h-[150px]">
                            {"main"}
                        </Main>
                    </Container>
                </dd>
                <dt class="text-4xl pb-4 pt-8">{"Style 2"}</dt>
                <dd>
                    <Container>
                        <Header class="bg-red-200 h-[60px]">
                            {"header"}
                        </Header>
                        <Main class="bg-blue-200 h-[150px]">
                            {"main"}
                        </Main>
                        <Footer class="bg-gray-200 h-[40px]">
                        {"footer"}
                        </Footer>
                    </Container>
                </dd>
                <dt class="text-4xl pb-4 pt-8">{"Style 3"}</dt>
                <dd>
                    <Container>
                        <Header class="bg-red-200 h-[60px]">
                            {"header"}
                        </Header>
                        <Container direction={Direction::Row}>
                            <Aside class="bg-sky-200 h-[150px]" width={"400px"}>
                                {"aside"}
                            </Aside>
                            <Main class="bg-blue-200 h-[150px] flex-1">
                                {"main"}
                            </Main>
                        </Container>
                    </Container>
                </dd>
            </dl>
            <CFooter prev={"/docs"} next={"/docs"} />
        </div>
    }
}
