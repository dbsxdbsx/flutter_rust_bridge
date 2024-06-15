"use strict";(self.webpackChunkflutter_rust_bridge=self.webpackChunkflutter_rust_bridge||[]).push([[4716],{3905:(e,t,r)=>{r.d(t,{Zo:()=>p,kt:()=>c});var n=r(7294);function i(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function a(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,n)}return r}function o(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?a(Object(r),!0).forEach((function(t){i(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):a(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function d(e,t){if(null==e)return{};var r,n,i=function(e,t){if(null==e)return{};var r,n,i={},a=Object.keys(e);for(n=0;n<a.length;n++)r=a[n],t.indexOf(r)>=0||(i[r]=e[r]);return i}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(n=0;n<a.length;n++)r=a[n],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(i[r]=e[r])}return i}var l=n.createContext({}),s=function(e){var t=n.useContext(l),r=t;return e&&(r="function"==typeof e?e(t):o(o({},t),e)),r},p=function(e){var t=s(e.components);return n.createElement(l.Provider,{value:t},e.children)},u={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},m=n.forwardRef((function(e,t){var r=e.components,i=e.mdxType,a=e.originalType,l=e.parentName,p=d(e,["components","mdxType","originalType","parentName"]),m=s(r),c=i,h=m["".concat(l,".").concat(c)]||m[c]||u[c]||a;return r?n.createElement(h,o(o({ref:t},p),{},{components:r})):n.createElement(h,o({ref:t},p))}));function c(e,t){var r=arguments,i=t&&t.mdxType;if("string"==typeof e||i){var a=r.length,o=new Array(a);o[0]=m;var d={};for(var l in t)hasOwnProperty.call(t,l)&&(d[l]=t[l]);d.originalType=e,d.mdxType="string"==typeof e?e:i,o[1]=d;for(var s=2;s<a;s++)o[s]=r[s];return n.createElement.apply(null,o)}return n.createElement.apply(null,r)}m.displayName="MDXCreateElement"},6594:(e,t,r)=>{r.r(t),r.d(t,{assets:()=>l,contentTitle:()=>o,default:()=>u,frontMatter:()=>a,metadata:()=>d,toc:()=>s});var n=r(7462),i=(r(7294),r(3905));const a={},o="Override/add methods",d={unversionedId:"guides/third-party/automatic/override-methods",id:"guides/third-party/automatic/override-methods",title:"Override/add methods",description:"In this page, we show how to do these in the example below:",source:"@site/docs/guides/third-party/automatic/override-methods.md",sourceDirName:"guides/third-party/automatic",slug:"/guides/third-party/automatic/override-methods",permalink:"/flutter_rust_bridge/guides/third-party/automatic/override-methods",draft:!1,editUrl:"https://github.com/fzyzcjy/flutter_rust_bridge/tree/master/website/docs/guides/third-party/automatic/override-methods.md",tags:[],version:"current",frontMatter:{},sidebar:"tutorialSidebar",previous:{title:"Override attributes",permalink:"/flutter_rust_bridge/guides/third-party/automatic/override-attributes"},next:{title:"Tricks",permalink:"/flutter_rust_bridge/guides/third-party/automatic/tricks"}},l={},s=[{value:"(Optional) How it is done",id:"optional-how-it-is-done",level:2},{value:"Example",id:"example",level:2},{value:"Override existing methods",id:"override-existing-methods",level:3},{value:"Add new methods",id:"add-new-methods",level:3}],p={toc:s};function u(e){let{components:t,...r}=e;return(0,i.kt)("wrapper",(0,n.Z)({},p,r,{components:t,mdxType:"MDXLayout"}),(0,i.kt)("h1",{id:"overrideadd-methods"},"Override/add methods"),(0,i.kt)("p",null,"In this page, we show how to do these in the example below:"),(0,i.kt)("ul",null,(0,i.kt)("li",{parentName:"ul"},"Change the function signature / method implementation / ... of something in a third-party crate"),(0,i.kt)("li",{parentName:"ul"},"Add arbitrary methods to existing structs/enums in third party crates")),(0,i.kt)("h2",{id:"optional-how-it-is-done"},"(Optional) How it is done"),(0,i.kt)("p",null,"How is the example below implemented?\nShortly speaking,\nthe ",(0,i.kt)("inlineCode",{parentName:"p"},"#[ext]"),' macro (which implements the "extension trait pattern") automatically generates a trait and an implementation,\nwhich ',(0,i.kt)("inlineCode",{parentName:"p"},"flutter_rust_bridge")," picks up.\nThen, the ",(0,i.kt)("a",{parentName:"p",href:"../../miscellaneous/override-prefix"},(0,i.kt)("inlineCode",{parentName:"a"},"frb_override_")," prefix")," is recognized to automatically rename and override the original function."),(0,i.kt)("h2",{id:"example"},"Example"),(0,i.kt)("h3",{id:"override-existing-methods"},"Override existing methods"),(0,i.kt)("p",null,"Suppose the third party crate has code like:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-rust"},"pub struct S { ... }\nimpl S {\n    pub fn greet(&self, name: &str) { ... }\n}\n")),(0,i.kt)("p",null,"Then we can override the ",(0,i.kt)("inlineCode",{parentName:"p"},"greet")," function like:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-rust"},'use extend::ext; // or, for example, easy_ext\'s\nuse the_external_crate::path::to::S;\n\n#[ext]\npub impl S {\n    pub fn frb_override_greet(&self, age: i32, first_name: String, last_name: Vec<u8>) {\n        // We can have arbitrary implementation.\n        // Here, we demonstrate how to call the original implementation with modified arguments.\n        self.greet(format!("{age}-{first_name}-{last_name:?}"))\n    }\n}\n')),(0,i.kt)("h3",{id:"add-new-methods"},"Add new methods"),(0,i.kt)("p",null,"It is very similar to the approach above, except that we do not need to prefix with ",(0,i.kt)("inlineCode",{parentName:"p"},"frb_override_"),"."))}u.isMDXComponent=!0}}]);