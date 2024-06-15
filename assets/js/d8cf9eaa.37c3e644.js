"use strict";(self.webpackChunkflutter_rust_bridge=self.webpackChunkflutter_rust_bridge||[]).push([[1290],{3905:(e,t,r)=>{r.d(t,{Zo:()=>c,kt:()=>d});var n=r(7294);function a(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function l(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,n)}return r}function i(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?l(Object(r),!0).forEach((function(t){a(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):l(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function o(e,t){if(null==e)return{};var r,n,a=function(e,t){if(null==e)return{};var r,n,a={},l=Object.keys(e);for(n=0;n<l.length;n++)r=l[n],t.indexOf(r)>=0||(a[r]=e[r]);return a}(e,t);if(Object.getOwnPropertySymbols){var l=Object.getOwnPropertySymbols(e);for(n=0;n<l.length;n++)r=l[n],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(a[r]=e[r])}return a}var u=n.createContext({}),s=function(e){var t=n.useContext(u),r=t;return e&&(r="function"==typeof e?e(t):i(i({},t),e)),r},c=function(e){var t=s(e.components);return n.createElement(u.Provider,{value:t},e.children)},f={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},p=n.forwardRef((function(e,t){var r=e.components,a=e.mdxType,l=e.originalType,u=e.parentName,c=o(e,["components","mdxType","originalType","parentName"]),p=s(r),d=a,m=p["".concat(u,".").concat(d)]||p[d]||f[d]||l;return r?n.createElement(m,i(i({ref:t},c),{},{components:r})):n.createElement(m,i({ref:t},c))}));function d(e,t){var r=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var l=r.length,i=new Array(l);i[0]=p;var o={};for(var u in t)hasOwnProperty.call(t,u)&&(o[u]=t[u]);o.originalType=e,o.mdxType="string"==typeof e?e:a,i[1]=o;for(var s=2;s<l;s++)i[s]=r[s];return n.createElement.apply(null,i)}return n.createElement.apply(null,r)}p.displayName="MDXCreateElement"},2248:(e,t,r)=>{r.r(t),r.d(t,{assets:()=>u,contentTitle:()=>i,default:()=>f,frontMatter:()=>l,metadata:()=>o,toc:()=>s});var n=r(7462),a=(r(7294),r(3905));const l={},i="Parameter defaults",o={unversionedId:"guides/miscellaneous/default",id:"guides/miscellaneous/default",title:"Parameter defaults",description:"Dart allows default values for function and constructor parameters, and you can achieve the same effect using #[frb(default)]. The syntax is as follows:",source:"@site/docs/guides/miscellaneous/default.md",sourceDirName:"guides/miscellaneous",slug:"/guides/miscellaneous/default",permalink:"/flutter_rust_bridge/guides/miscellaneous/default",draft:!1,editUrl:"https://github.com/fzyzcjy/flutter_rust_bridge/tree/master/website/docs/guides/miscellaneous/default.md",tags:[],version:"current",frontMatter:{},sidebar:"tutorialSidebar",previous:{title:"Expanding macros",permalink:"/flutter_rust_bridge/guides/miscellaneous/expanding-macros"},next:{title:"Multiple input folders",permalink:"/flutter_rust_bridge/guides/miscellaneous/multi-input"}},u={},s=[],c={toc:s};function f(e){let{components:t,...r}=e;return(0,a.kt)("wrapper",(0,n.Z)({},c,r,{components:t,mdxType:"MDXLayout"}),(0,a.kt)("h1",{id:"parameter-defaults"},"Parameter defaults"),(0,a.kt)("p",null,"Dart allows default values for function and constructor parameters, and you can achieve the same effect using ",(0,a.kt)("inlineCode",{parentName:"p"},"#[frb(default)]"),". The syntax is as follows:"),(0,a.kt)("ul",null,(0,a.kt)("li",{parentName:"ul"},"If the parameter is a ",(0,a.kt)("inlineCode",{parentName:"li"},"String")," or any other primitive, ",(0,a.kt)("inlineCode",{parentName:"li"},'#[frb(default = ".." | 0 | true | ..)]')," annotates its default value."),(0,a.kt)("li",{parentName:"ul"},"If the parameter is a class or an enum, ",(0,a.kt)("inlineCode",{parentName:"li"},'#[frb(default = "..")]')," annotates the ",(0,a.kt)("em",{parentName:"li"},"Dart code")," to initialize the parameter.\nNote that this is run in the ",(0,a.kt)("em",{parentName:"li"},"constant context"),", so classes can only be constructed if they are preceded with ",(0,a.kt)("inlineCode",{parentName:"li"},"const"),".")),(0,a.kt)("p",null,"This will be translated to either a default value annotation, or Freezed's ",(0,a.kt)("inlineCode",{parentName:"p"},"@Default")," in the case of enum constructor parameters."),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},'pub enum Answer { Yes, No }\npub struct Point(pub f64, pub f64);\n\n#[frb]\npub fn defaults(\n    #[frb(default = "Answer.Yes")]\n    answer: Answer,\n    #[frb(default = "const Point(field0: 2, field1: 3)")]\n    point: Point,\n);\n')))}f.isMDXComponent=!0}}]);