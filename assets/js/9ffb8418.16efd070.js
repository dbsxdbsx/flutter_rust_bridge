"use strict";(self.webpackChunkflutter_rust_bridge=self.webpackChunkflutter_rust_bridge||[]).push([[9806],{3905:(e,t,r)=>{r.d(t,{Zo:()=>u,kt:()=>m});var n=r(7294);function a(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function o(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,n)}return r}function l(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?o(Object(r),!0).forEach((function(t){a(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):o(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function i(e,t){if(null==e)return{};var r,n,a=function(e,t){if(null==e)return{};var r,n,a={},o=Object.keys(e);for(n=0;n<o.length;n++)r=o[n],t.indexOf(r)>=0||(a[r]=e[r]);return a}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(n=0;n<o.length;n++)r=o[n],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(a[r]=e[r])}return a}var s=n.createContext({}),c=function(e){var t=n.useContext(s),r=t;return e&&(r="function"==typeof e?e(t):l(l({},t),e)),r},u=function(e){var t=c(e.components);return n.createElement(s.Provider,{value:t},e.children)},p={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},d=n.forwardRef((function(e,t){var r=e.components,a=e.mdxType,o=e.originalType,s=e.parentName,u=i(e,["components","mdxType","originalType","parentName"]),d=c(r),m=a,y=d["".concat(s,".").concat(m)]||d[m]||p[m]||o;return r?n.createElement(y,l(l({ref:t},u),{},{components:r})):n.createElement(y,l({ref:t},u))}));function m(e,t){var r=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var o=r.length,l=new Array(o);l[0]=d;var i={};for(var s in t)hasOwnProperty.call(t,s)&&(i[s]=t[s]);i.originalType=e,i.mdxType="string"==typeof e?e:a,l[1]=i;for(var c=2;c<o;c++)l[c]=r[c];return n.createElement.apply(null,l)}return n.createElement.apply(null,r)}d.displayName="MDXCreateElement"},7712:(e,t,r)=>{r.r(t),r.d(t,{assets:()=>s,contentTitle:()=>l,default:()=>p,frontMatter:()=>o,metadata:()=>i,toc:()=>c});var n=r(7462),a=(r(7294),r(3905));const o={},l="Custom encoder/decoders",i={unversionedId:"guides/types/translatable/custom",id:"guides/types/translatable/custom",title:"Custom encoder/decoders",description:"We can customize how a type is encoded and decoded.",source:"@site/docs/guides/types/translatable/custom.md",sourceDirName:"guides/types/translatable",slug:"/guides/types/translatable/custom",permalink:"/flutter_rust_bridge/guides/types/translatable/custom",draft:!1,editUrl:"https://github.com/fzyzcjy/flutter_rust_bridge/tree/master/website/docs/guides/types/translatable/custom.md",tags:[],version:"current",frontMatter:{},sidebar:"tutorialSidebar",previous:{title:"Stream / Iterator",permalink:"/flutter_rust_bridge/guides/types/translatable/stream"},next:{title:"Zero copy",permalink:"/flutter_rust_bridge/guides/types/translatable/zero-copy"}},s={},c=[{value:"Example",id:"example",level:2},{value:"Remarks",id:"remarks",level:2}],u={toc:c};function p(e){let{components:t,...r}=e;return(0,a.kt)("wrapper",(0,n.Z)({},u,r,{components:t,mdxType:"MDXLayout"}),(0,a.kt)("h1",{id:"custom-encoderdecoders"},"Custom encoder/decoders"),(0,a.kt)("p",null,"We can customize how a type is encoded and decoded."),(0,a.kt)("h2",{id:"example"},"Example"),(0,a.kt)("p",null,"Suppose we have a ",(0,a.kt)("inlineCode",{parentName:"p"},"FancyRustType")," and corresponding type ",(0,a.kt)("inlineCode",{parentName:"p"},"FancyDartType"),"\n(they can be the same name, here we use different names just to make it clearer).\nIf we want to encode it using a ",(0,a.kt)("inlineCode",{parentName:"p"},"String")," (can use arbitrary complex types as long as flutter_rust_bridge supports),\nthen we can write code like below:"),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},'#[frb(rust2dart(dart_type = "FancyDartType", dart_code = "FancyDartType.letsParseIt({})"))]\npub fn encode_fancy_type(raw: FancyRustType) -> String { ... }\n\n#[frb(dart2rust(dart_type = "FancyDartType", dart_code = "{}.letsEncodeIt()"))]\npub fn decode_fancy_type(raw: String) -> FancyRustType { ... }\n')),(0,a.kt)("p",null,"The function names above are arbitrarily chosen.\nThen, whenever we are using ",(0,a.kt)("inlineCode",{parentName:"p"},"FancyType"),", such as:"),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},"pub fn f(a: FancyRustType) { ... }\n")),(0,a.kt)("p",null,"It will be automatically converted to:"),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-dart"},"void f(FancyDartType a) { ... }\n")),(0,a.kt)("p",null,"And under the hood, the type will be encoded/decoded via the custom functions."),(0,a.kt)("h2",{id:"remarks"},"Remarks"),(0,a.kt)("ul",null,(0,a.kt)("li",{parentName:"ul"},"If the Dart types need some ",(0,a.kt)("inlineCode",{parentName:"li"},"import"),"s to work, the ",(0,a.kt)("inlineCode",{parentName:"li"},"dart_preamble")," config key in ",(0,a.kt)("inlineCode",{parentName:"li"},"flutter_rust_bridge.yaml")," can be utilized to import things."),(0,a.kt)("li",{parentName:"ul"},"If the encoding/decoding process returns a ",(0,a.kt)("inlineCode",{parentName:"li"},"Result"),", currently you can ",(0,a.kt)("inlineCode",{parentName:"li"},"unwrap()")," it to convert it to a panic.")))}p.isMDXComponent=!0}}]);