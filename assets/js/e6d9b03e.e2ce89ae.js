"use strict";(self.webpackChunkflutter_rust_bridge=self.webpackChunkflutter_rust_bridge||[]).push([[2696],{3905:(e,t,n)=>{n.d(t,{Zo:()=>u,kt:()=>m});var r=n(7294);function a(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function o(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function i(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?o(Object(n),!0).forEach((function(t){a(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):o(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function l(e,t){if(null==e)return{};var n,r,a=function(e,t){if(null==e)return{};var n,r,a={},o=Object.keys(e);for(r=0;r<o.length;r++)n=o[r],t.indexOf(n)>=0||(a[n]=e[n]);return a}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(r=0;r<o.length;r++)n=o[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(a[n]=e[n])}return a}var p=r.createContext({}),c=function(e){var t=r.useContext(p),n=t;return e&&(n="function"==typeof e?e(t):i(i({},t),e)),n},u=function(e){var t=c(e.components);return r.createElement(p.Provider,{value:t},e.children)},s={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},d=r.forwardRef((function(e,t){var n=e.components,a=e.mdxType,o=e.originalType,p=e.parentName,u=l(e,["components","mdxType","originalType","parentName"]),d=c(n),m=a,g=d["".concat(p,".").concat(m)]||d[m]||s[m]||o;return n?r.createElement(g,i(i({ref:t},u),{},{components:n})):r.createElement(g,i({ref:t},u))}));function m(e,t){var n=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var o=n.length,i=new Array(o);i[0]=d;var l={};for(var p in t)hasOwnProperty.call(t,p)&&(l[p]=t[p]);l.originalType=e,l.mdxType="string"==typeof e?e:a,i[1]=l;for(var c=2;c<o;c++)i[c]=n[c];return r.createElement.apply(null,i)}return r.createElement.apply(null,n)}d.displayName="MDXCreateElement"},4787:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>p,contentTitle:()=>i,default:()=>s,frontMatter:()=>o,metadata:()=>l,toc:()=>c});var r=n(7462),a=(n(7294),n(3905));const o={},i="Adding new code",l={unversionedId:"manual/integrate/template/generate/adding-code",id:"manual/integrate/template/generate/adding-code",title:"Adding new code",description:"Let's say we need to change Platform such that we don't really care about whether it",source:"@site/docs/manual/integrate/05-template/03-generate/02-adding-code.md",sourceDirName:"manual/integrate/05-template/03-generate",slug:"/manual/integrate/template/generate/adding-code",permalink:"/flutter_rust_bridge/manual/integrate/template/generate/adding-code",draft:!1,editUrl:"https://github.com/fzyzcjy/flutter_rust_bridge/tree/master/website/docs/manual/integrate/05-template/03-generate/02-adding-code.md",tags:[],version:"current",sidebarPosition:2,frontMatter:{},sidebar:"tutorialSidebar",previous:{title:"Installing codegen",permalink:"/flutter_rust_bridge/manual/integrate/template/generate/install"},next:{title:"Using build_runner",permalink:"/flutter_rust_bridge/manual/integrate/template/generate/build-runner"}},p={},c=[{value:"Troubleshooting: &quot;Please supply one or more path/to/llvm...&quot;",id:"troubleshooting-please-supply-one-or-more-pathtollvm",level:2}],u={toc:c};function s(e){let{components:t,...n}=e;return(0,a.kt)("wrapper",(0,r.Z)({},u,n,{components:t,mdxType:"MDXLayout"}),(0,a.kt)("h1",{id:"adding-new-code"},"Adding new code"),(0,a.kt)("p",null,"Let's say we need to change ",(0,a.kt)("inlineCode",{parentName:"p"},"Platform")," such that we don't really care about whether it\nis running on Intel or Apple Silicon, but we would like to keep this information so\ndownlevel code can act on it. We would like to merge ",(0,a.kt)("inlineCode",{parentName:"p"},"MacApple")," and ",(0,a.kt)("inlineCode",{parentName:"p"},"MacIntel")," into a\nsingle ",(0,a.kt)("inlineCode",{parentName:"p"},"MacOs(String)")," that contains the current CPU architecture. Go ahead and update\n",(0,a.kt)("inlineCode",{parentName:"p"},"native/src/api.rs"),":"),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-diff"}," pub enum Platform {\n     ..\n-    MacIntel,\n-    MacApple,\n+    MacOs(String),\n     ..\n }\n")),(0,a.kt)("p",null,"Now run ",(0,a.kt)("inlineCode",{parentName:"p"},"just")," and see that your binding code now has changed."),(0,a.kt)("h2",{id:"troubleshooting-please-supply-one-or-more-pathtollvm"},'Troubleshooting: "Please supply one or more path/to/llvm..."'),(0,a.kt)("p",null,"A common issue with ",(0,a.kt)("inlineCode",{parentName:"p"},"ffigen")," is that its detection of the LLVM installation is not reliable\nacross platforms. Especially for MacOS and the split between x86-64 and arm64 binaries,\nyou might have to modify ",(0,a.kt)("inlineCode",{parentName:"p"},"justfile")," to explicitly point to its location:"),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre"},'llvm_path := if os() == "macos" {\n    "--llvm-path /opt/homebrew/opt/llvm"\n} else {\n    ""\n}\n')))}s.isMDXComponent=!0}}]);