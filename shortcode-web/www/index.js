import * as wasm from "shortcode-web";

const input = document.getElementById("lookup");

input.onkeydown = function(e) {
    if (e.key == "Enter") {
        const ret = wasm.shortcode(e.target.value);

        console.log(ret)

        let txt = document.createElement("p");
        if (!ret.found) {
            txt.innerText = "not found";
        } else {
            txt.innerText = [ret.first, ret.second, ret.third].join('');
        }

        e.target.value = "";
        document.body.appendChild(txt);
    }
}
