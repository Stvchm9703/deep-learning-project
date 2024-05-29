use leptonic::prelude::*;

use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn MainCoverView() -> impl IntoView {
    view! {
    <Box class="view main-cover-view">
        <div class="main-cover-icon">
            <img src="public/logo.jpg" />
        </div>
         <div class="main-cover-box">
             <div class="main-cover-box__title">
                 <h1>
                    Start your hair<br/>
                    journey with<br/>
                    <span class="highlight">us. </span>
                 </h1>
                 <p>Applying technology to analyse your face shape and <br/> provide the best hairstyle for you.</p>
             </div>
             <img class="main-cover-box__image" src="public/cover_img.jpg" />
         </div>
    </Box>
    }
}
