import * as wasm from "shortcode-web";

const input = document.getElementById("lookup");

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

// https://coolaj86.com/articles/convert-js-bigints-to-typedarrays/
function bnToBuf(bn) {
  var hex = BigInt(bn).toString(16);
  if (hex.length % 2) { hex = '0' + hex; }

  var len = hex.length / 2;
  var u8 = new Uint8Array(len);

  var i = 0;
  var j = 0;
  while (i < len) {
    u8[i] = parseInt(hex.slice(j, j+2), 16);
    i += 1;
    j += 2;
  }

  return u8;
}

input.onkeydown = function(e) {
    if (e.key == "Enter") {
        const ret = wasm.lookup(trimChar(e.target.value, ':'));

        console.log(ret.toString(16))
        console.log(bnToBuf(ret))

        let txt = document.createElement("p");
        if (ret == 0) {
            txt.innerText = "not found";
        } else {
            let str = new TextDecoder().decode(bnToBuf(ret).reverse());
            txt.innerText = str;
        }

        e.target.value = "";
        document.body.appendChild(txt);
    }
}
