(()=>{"use strict";var e,t,r={},n={};function o(e){if(n[e])return n[e].exports;var t=n[e]={id:e,loaded:!1,exports:{}};return r[e](t,t.exports,o),t.loaded=!0,t.exports}function a(){return new Worker(o.p+"7f101432d4db7de12de5.worker.js")}o.m=r,o.d=(e,t)=>{for(var r in t)o.o(t,r)&&!o.o(e,r)&&Object.defineProperty(e,r,{enumerable:!0,get:t[r]})},o.f={},o.e=e=>Promise.all(Object.keys(o.f).reduce(((t,r)=>(o.f[r](e,t),t)),[])),o.u=e=>"78cea21a6d4da4076ace.js",o.miniCssF=e=>235===e?"dbd6777cfd0b678d50c7.css":void 0+".css",o.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),o.hmd=e=>((e=Object.create(e)).children||(e.children=[]),Object.defineProperty(e,"exports",{enumerable:!0,set:()=>{throw new Error("ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: "+e.id)}}),e),o.o=(e,t)=>Object.prototype.hasOwnProperty.call(e,t),e={},t="sudoku:",o.l=(r,n,a)=>{if(e[r])e[r].push(n);else{var s,i;if(void 0!==a)for(var d=document.getElementsByTagName("script"),u=0;u<d.length;u++){var c=d[u];if(c.getAttribute("src")==r||c.getAttribute("data-webpack")==t+a){s=c;break}}s||(i=!0,(s=document.createElement("script")).charset="utf-8",s.timeout=120,o.nc&&s.setAttribute("nonce",o.nc),s.setAttribute("data-webpack",t+a),s.src=r),e[r]=[n];var l=(t,n)=>{s.onerror=s.onload=null,clearTimeout(p);var o=e[r];if(delete e[r],s.parentNode&&s.parentNode.removeChild(s),o&&o.forEach((e=>e(n))),t)return t(n)},p=setTimeout(l.bind(null,void 0,{type:"timeout",target:s}),12e4);s.onerror=l.bind(null,s.onerror),s.onload=l.bind(null,s.onload),i&&document.head.appendChild(s)}},o.r=e=>{"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},(()=>{var e;o.g.importScripts&&(e=o.g.location+"");var t=o.g.document;if(!e&&t&&(t.currentScript&&(e=t.currentScript.src),!e)){var r=t.getElementsByTagName("script");r.length&&(e=r[r.length-1].src)}if(!e)throw new Error("Automatic publicPath is not supported in this browser");e=e.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),o.p=e})(),(()=>{var e={826:0};o.f.j=(t,r)=>{var n=o.o(e,t)?e[t]:void 0;if(0!==n)if(n)r.push(n[2]);else{var a=new Promise(((r,o)=>{n=e[t]=[r,o]}));r.push(n[2]=a);var s=o.p+o.u(t),i=new Error;o.l(s,(r=>{if(o.o(e,t)&&(0!==(n=e[t])&&(e[t]=void 0),n)){var a=r&&("load"===r.type?"missing":r.type),s=r&&r.target&&r.target.src;i.message="Loading chunk "+t+" failed.\n("+a+": "+s+")",i.name="ChunkLoadError",i.type=a,i.request=s,n[1](i)}}),"chunk-"+t)}};var t=self.webpackChunksudoku=self.webpackChunksudoku||[],r=t.push.bind(t);t.push=t=>{for(var n,a,[s,i,d]=t,u=0,c=[];u<s.length;u++)a=s[u],o.o(e,a)&&e[a]&&c.push(e[a][0]),e[a]=0;for(n in i)o.o(i,n)&&(o.m[n]=i[n]);for(d&&d(o),r(t);c.length;)c.shift()()}})(),o.v=(e,t,r,n)=>{var a=fetch(o.p+""+r+".module.wasm");return"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(a,n).then((t=>Object.assign(e,t.instance.exports))):a.then((e=>e.arrayBuffer())).then((e=>WebAssembly.instantiate(e,n))).then((t=>Object.assign(e,t.instance.exports)))},o.e(235).then(o.bind(o,235)).then((e=>{let t=new a;t.onmessage=function(r){"init"==r.data[0]?(e.init(),e.set_solver((function(e){t.postMessage([e])})),e.start()):"solved"==r.data[0]&&(console.log(r.data[1]),e.on_solve(r.data[1]),e.on_measure(r.data[2]))},t.postMessage(["init"])}))})();