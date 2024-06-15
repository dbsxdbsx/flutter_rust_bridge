"use strict";(self.webpackChunkflutter_rust_bridge=self.webpackChunkflutter_rust_bridge||[]).push([[6113],{3905:(e,t,r)=>{r.d(t,{Zo:()=>l,kt:()=>y});var n=r(7294);function o(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function a(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,n)}return r}function c(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?a(Object(r),!0).forEach((function(t){o(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):a(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function u(e,t){if(null==e)return{};var r,n,o=function(e,t){if(null==e)return{};var r,n,o={},a=Object.keys(e);for(n=0;n<a.length;n++)r=a[n],t.indexOf(r)>=0||(o[r]=e[r]);return o}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(n=0;n<a.length;n++)r=a[n],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(o[r]=e[r])}return o}var s=n.createContext({}),i=function(e){var t=n.useContext(s),r=t;return e&&(r="function"==typeof e?e(t):c(c({},t),e)),r},l=function(e){var t=i(e.components);return n.createElement(s.Provider,{value:t},e.children)},d={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},p=n.forwardRef((function(e,t){var r=e.components,o=e.mdxType,a=e.originalType,s=e.parentName,l=u(e,["components","mdxType","originalType","parentName"]),p=i(r),y=o,f=p["".concat(s,".").concat(y)]||p[y]||d[y]||a;return r?n.createElement(f,c(c({ref:t},l),{},{components:r})):n.createElement(f,c({ref:t},l))}));function y(e,t){var r=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var a=r.length,c=new Array(a);c[0]=p;var u={};for(var s in t)hasOwnProperty.call(t,s)&&(u[s]=t[s]);u.originalType=e,u.mdxType="string"==typeof e?e:o,c[1]=u;for(var i=2;i<a;i++)c[i]=r[i];return n.createElement.apply(null,c)}return n.createElement.apply(null,r)}p.displayName="MDXCreateElement"},1021:(e,t,r)=>{r.r(t),r.d(t,{assets:()=>s,contentTitle:()=>c,default:()=>d,frontMatter:()=>a,metadata:()=>u,toc:()=>i});var n=r(7462),o=(r(7294),r(3905));const a={},c="Synchronous Dart",u={unversionedId:"guides/concurrency/sync-dart",id:"guides/concurrency/sync-dart",title:"Synchronous Dart",description:"In order to generate synchronous functions in Dart,",source:"@site/docs/guides/concurrency/sync-dart.md",sourceDirName:"guides/concurrency",slug:"/guides/concurrency/sync-dart",permalink:"/flutter_rust_bridge/guides/concurrency/sync-dart",draft:!1,editUrl:"https://github.com/fzyzcjy/flutter_rust_bridge/tree/master/website/docs/guides/concurrency/sync-dart.md",tags:[],version:"current",frontMatter:{},sidebar:"tutorialSidebar",previous:{title:"Asynchronous Rust",permalink:"/flutter_rust_bridge/guides/concurrency/async-rust"},next:{title:"Asynchronous Dart",permalink:"/flutter_rust_bridge/guides/concurrency/async-dart"}},s={},i=[{value:"Example",id:"example",level:2}],l={toc:i};function d(e){let{components:t,...r}=e;return(0,o.kt)("wrapper",(0,n.Z)({},l,r,{components:t,mdxType:"MDXLayout"}),(0,o.kt)("h1",{id:"synchronous-dart"},"Synchronous Dart"),(0,o.kt)("p",null,"In order to generate synchronous functions in Dart,\nadd ",(0,o.kt)("inlineCode",{parentName:"p"},"#[frb(sync)]")," attributes to the Rust function.\nAs usual, this feature is compatible with other features,\ne.g. you can use arbitrary argument and return types."),(0,o.kt)("p",null,"It is useful when you want to make the generated Dart API be synchronous,\nespecially helpful when you cannot ",(0,o.kt)("inlineCode",{parentName:"p"},"await")," something,\nfor example inside Flutter's ",(0,o.kt)("inlineCode",{parentName:"p"},"build"),"."),(0,o.kt)("p",null,"If your function is quick and is called a ton of times,\nusing synchronous mode will reduce the calling overhead (though already small).\nIf, on the other hand, your function is slow,\nit is recommended to use the asynchronous mode,\nbecause synchronous mode will block the Dart UI."),(0,o.kt)("h2",{id:"example"},"Example"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-rust"},"fn normal() {}\n\n#[frb(sync)]\nfn dart_counterpart_is_synchronous() {}\n")),(0,o.kt)("p",null,"Dart:"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-dart"},"await normal(); // Need await\ndartCounterpartIsSynchronous(); // No need await\n")))}d.isMDXComponent=!0}}]);