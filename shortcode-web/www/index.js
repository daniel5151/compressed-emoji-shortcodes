import * as wasm from "shortcode-web";

window.shortcode_lookup = wasm.lookup;

const input = document.getElementById("lookup");
const output = document.getElementById("output");
const sample_invalid_input = document.getElementById("sample");

// https://stackoverflow.com/questions/1349404/generate-random-string-characters-in-javascript
function makeid(length) {
   var result           = '';
   var characters       = 'abcdefghijklmnopqrstuvwxyz__';
   var charactersLength = characters.length;
   for ( var i = 0; i < length; i++ ) {
      result += characters.charAt(Math.floor(Math.random() * charactersLength));
   }
   return result;
}

// if the hash function didn't have false positives, this would loop forever.
// good thing it doesn't!
while (true) {
    const id = makeid(10);
    const out = shortcode_lookup(id);
    if (out) {
        sample_invalid_input.innerHTML = `<code>:${id}:</code> maps to ${out}`
        break;
    }
}

// https://stackoverflow.com/a/26156806
function trimChar(string, charToRemove) {
    while(string.charAt(0)==charToRemove) {
        string = string.substring(1);
    }

    while(string.charAt(string.length-1)==charToRemove) {
        string = string.substring(0,string.length-1);
    }

    return string;
}

input.onkeydown = function(e) {
    if (e.key == "Enter") {
        const val = trimChar(e.target.value, ':');
        const ret = wasm.lookup(val);

        let txt = "";
        if (!ret) {
            txt = `<p><code>:${val}:</code> is not a valid shortcode</p>`;
        } else {
            txt = `<p>mapped <code>:${val}:</code> to ${ret}</p>`;
        }

        e.target.value = "";
        output.innerHTML = txt + output.innerHTML;
    }
}
