"use strict";(self.webpackChunkflutter_rust_bridge=self.webpackChunkflutter_rust_bridge||[]).push([[8437],{3905:(e,t,r)=>{r.d(t,{Zo:()=>p,kt:()=>m});var n=r(7294);function a(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function i(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,n)}return r}function o(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?i(Object(r),!0).forEach((function(t){a(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):i(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function c(e,t){if(null==e)return{};var r,n,a=function(e,t){if(null==e)return{};var r,n,a={},i=Object.keys(e);for(n=0;n<i.length;n++)r=i[n],t.indexOf(r)>=0||(a[r]=e[r]);return a}(e,t);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(n=0;n<i.length;n++)r=i[n],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(a[r]=e[r])}return a}var u=n.createContext({}),s=function(e){var t=n.useContext(u),r=t;return e&&(r="function"==typeof e?e(t):o(o({},t),e)),r},p=function(e){var t=s(e.components);return n.createElement(u.Provider,{value:t},e.children)},l={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},d=n.forwardRef((function(e,t){var r=e.components,a=e.mdxType,i=e.originalType,u=e.parentName,p=c(e,["components","mdxType","originalType","parentName"]),d=s(r),m=a,f=d["".concat(u,".").concat(m)]||d[m]||l[m]||i;return r?n.createElement(f,o(o({ref:t},p),{},{components:r})):n.createElement(f,o({ref:t},p))}));function m(e,t){var r=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var i=r.length,o=new Array(i);o[0]=d;var c={};for(var u in t)hasOwnProperty.call(t,u)&&(c[u]=t[u]);c.originalType=e,c.mdxType="string"==typeof e?e:a,o[1]=c;for(var s=2;s<i;s++)o[s]=r[s];return n.createElement.apply(null,o)}return n.createElement.apply(null,r)}d.displayName="MDXCreateElement"},8609:(e,t,r)=>{r.r(t),r.d(t,{assets:()=>u,contentTitle:()=>o,default:()=>l,frontMatter:()=>i,metadata:()=>c,toc:()=>s});var n=r(7462),a=(r(7294),r(3905));const i={},o="Scanning",c={unversionedId:"guides/third-party/automatic/scanning",id:"guides/third-party/automatic/scanning",title:"Scanning",description:"The first step is to configure to scan the third-party crate.",source:"@site/docs/guides/third-party/automatic/scanning.md",sourceDirName:"guides/third-party/automatic",slug:"/guides/third-party/automatic/scanning",permalink:"/flutter_rust_bridge/guides/third-party/automatic/scanning",draft:!1,editUrl:"https://github.com/fzyzcjy/flutter_rust_bridge/tree/master/website/docs/guides/third-party/automatic/scanning.md",tags:[],version:"current",frontMatter:{},sidebar:"tutorialSidebar",previous:{title:"Automatic",permalink:"/flutter_rust_bridge/guides/third-party/automatic/"},next:{title:"Override attributes",permalink:"/flutter_rust_bridge/guides/third-party/automatic/override-attributes"}},u={},s=[],p={toc:s};function l(e){let{components:t,...r}=e;return(0,a.kt)("wrapper",(0,n.Z)({},p,r,{components:t,mdxType:"MDXLayout"}),(0,a.kt)("h1",{id:"scanning"},"Scanning"),(0,a.kt)("p",null,"The first step is to configure to scan the third-party crate.\nThis is fairly simple - just modify ",(0,a.kt)("inlineCode",{parentName:"p"},"flutter_rust_bridge.yaml")," and change to something like:"),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-yaml"},"rust_input: crate::api,interesting_third_party_crate_name\n")),(0,a.kt)("p",null,"The line above means we want to both scan ",(0,a.kt)("inlineCode",{parentName:"p"},"src/api")," folder in our crate and scan the ",(0,a.kt)("inlineCode",{parentName:"p"},"interesting_third_party_crate_name")," crate."),(0,a.kt)("p",null,"For crate with ",(0,a.kt)("inlineCode",{parentName:"p"},"-")," in the name, we can write ",(0,a.kt)("inlineCode",{parentName:"p"},"interesting-third-party-crate-name")),(0,a.kt)("p",null,"Please refer to ",(0,a.kt)("a",{parentName:"p",href:"../../miscellaneous/multi-input"},"this page")," for more details of the configuration."))}l.isMDXComponent=!0}}]);