"use strict";(self.webpackChunkflutter_rust_bridge=self.webpackChunkflutter_rust_bridge||[]).push([[8204],{3905:(e,t,r)=>{r.d(t,{Zo:()=>l,kt:()=>d});var o=r(7294);function n(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function i(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);t&&(o=o.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,o)}return r}function a(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?i(Object(r),!0).forEach((function(t){n(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):i(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function u(e,t){if(null==e)return{};var r,o,n=function(e,t){if(null==e)return{};var r,o,n={},i=Object.keys(e);for(o=0;o<i.length;o++)r=i[o],t.indexOf(r)>=0||(n[r]=e[r]);return n}(e,t);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(o=0;o<i.length;o++)r=i[o],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(n[r]=e[r])}return n}var s=o.createContext({}),c=function(e){var t=o.useContext(s),r=t;return e&&(r="function"==typeof e?e(t):a(a({},t),e)),r},l=function(e){var t=c(e.components);return o.createElement(s.Provider,{value:t},e.children)},p={inlineCode:"code",wrapper:function(e){var t=e.children;return o.createElement(o.Fragment,{},t)}},m=o.forwardRef((function(e,t){var r=e.components,n=e.mdxType,i=e.originalType,s=e.parentName,l=u(e,["components","mdxType","originalType","parentName"]),m=c(r),d=n,f=m["".concat(s,".").concat(d)]||m[d]||p[d]||i;return r?o.createElement(f,a(a({ref:t},l),{},{components:r})):o.createElement(f,a({ref:t},l))}));function d(e,t){var r=arguments,n=t&&t.mdxType;if("string"==typeof e||n){var i=r.length,a=new Array(i);a[0]=m;var u={};for(var s in t)hasOwnProperty.call(t,s)&&(u[s]=t[s]);u.originalType=e,u.mdxType="string"==typeof e?e:n,a[1]=u;for(var c=2;c<i;c++)a[c]=r[c];return o.createElement.apply(null,a)}return o.createElement.apply(null,r)}m.displayName="MDXCreateElement"},9698:(e,t,r)=>{r.r(t),r.d(t,{assets:()=>s,contentTitle:()=>a,default:()=>p,frontMatter:()=>i,metadata:()=>u,toc:()=>c});var o=r(7462),n=(r(7294),r(3905));const i={},a="Rust compilation",u={unversionedId:"guides/how-to/rust-compilation",id:"guides/how-to/rust-compilation",title:"Rust compilation",description:"Sometimes, some customization about Rust compilation may be needed,",source:"@site/docs/guides/how-to/rust-compilation.md",sourceDirName:"guides/how-to",slug:"/guides/how-to/rust-compilation",permalink:"/flutter_rust_bridge/guides/how-to/rust-compilation",draft:!1,editUrl:"https://github.com/fzyzcjy/flutter_rust_bridge/tree/master/website/docs/guides/how-to/rust-compilation.md",tags:[],version:"current",frontMatter:{},sidebar:"tutorialSidebar",previous:{title:"Borrowed types",permalink:"/flutter_rust_bridge/guides/how-to/borrowed"},next:{title:"Cargo Workspaces",permalink:"/flutter_rust_bridge/guides/how-to/cargo-workspaces"}},s={},c=[],l={toc:c};function p(e){let{components:t,...r}=e;return(0,n.kt)("wrapper",(0,o.Z)({},l,r,{components:t,mdxType:"MDXLayout"}),(0,n.kt)("h1",{id:"rust-compilation"},"Rust compilation"),(0,n.kt)("p",null,"Sometimes, some customization about Rust compilation may be needed,\nsuch as ",(0,n.kt)("a",{parentName:"p",href:"https://github.com/fzyzcjy/flutter_rust_bridge/issues/1862"},"using nightly Rust")," (instead of stable Rust)."),(0,n.kt)("p",null,"To customize this, please refer to the documentations of the tool that is used in your project to compile Rust.\nSee ",(0,n.kt)("a",{parentName:"p",href:"../../manual/integrate/overview"},"here")," for a list of common tools.\nFor example, the default ",(0,n.kt)("inlineCode",{parentName:"p"},"flutter_rust_bridge_codegen create")," command uses Cargokit to compile Rust.\nThus, we can refer\nto ",(0,n.kt)("a",{parentName:"p",href:"https://github.com/irondash/cargokit/blob/main/docs/architecture.md#configuring-cargokit"},"the doc")," and write down:"),(0,n.kt)("pre",null,(0,n.kt)("code",{parentName:"pre",className:"language-yaml"},"# cargokit.yaml\ncargo:\n  release:\n    toolchain: nightly\n")),(0,n.kt)("p",null,"to fulfill the need of using nightly Rust."))}p.isMDXComponent=!0}}]);