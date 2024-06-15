"use strict";(self.webpackChunkflutter_rust_bridge=self.webpackChunkflutter_rust_bridge||[]).push([[4631],{3905:(e,t,r)=>{r.d(t,{Zo:()=>p,kt:()=>m});var a=r(7294);function n(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function o(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);t&&(a=a.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,a)}return r}function i(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?o(Object(r),!0).forEach((function(t){n(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):o(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function l(e,t){if(null==e)return{};var r,a,n=function(e,t){if(null==e)return{};var r,a,n={},o=Object.keys(e);for(a=0;a<o.length;a++)r=o[a],t.indexOf(r)>=0||(n[r]=e[r]);return n}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(a=0;a<o.length;a++)r=o[a],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(n[r]=e[r])}return n}var u=a.createContext({}),s=function(e){var t=a.useContext(u),r=t;return e&&(r="function"==typeof e?e(t):i(i({},t),e)),r},p=function(e){var t=s(e.components);return a.createElement(u.Provider,{value:t},e.children)},d={inlineCode:"code",wrapper:function(e){var t=e.children;return a.createElement(a.Fragment,{},t)}},c=a.forwardRef((function(e,t){var r=e.components,n=e.mdxType,o=e.originalType,u=e.parentName,p=l(e,["components","mdxType","originalType","parentName"]),c=s(r),m=n,f=c["".concat(u,".").concat(m)]||c[m]||d[m]||o;return r?a.createElement(f,i(i({ref:t},p),{},{components:r})):a.createElement(f,i({ref:t},p))}));function m(e,t){var r=arguments,n=t&&t.mdxType;if("string"==typeof e||n){var o=r.length,i=new Array(o);i[0]=c;var l={};for(var u in t)hasOwnProperty.call(t,u)&&(l[u]=t[u]);l.originalType=e,l.mdxType="string"==typeof e?e:n,i[1]=l;for(var s=2;s<o;s++)i[s]=r[s];return a.createElement.apply(null,i)}return a.createElement.apply(null,r)}c.displayName="MDXCreateElement"},5842:(e,t,r)=>{r.r(t),r.d(t,{assets:()=>u,contentTitle:()=>i,default:()=>d,frontMatter:()=>o,metadata:()=>l,toc:()=>s});var a=r(7462),n=(r(7294),r(3905));const o={},i="Tutorial: Pure Dart",l={unversionedId:"manual/miscellaneous/archived/tutorial-pure-dart",id:"manual/miscellaneous/archived/tutorial-pure-dart",title:"Tutorial: Pure Dart",description:"Remark: The valgrindtest section of the CI workflow can also be useful, if you want details of each command and want to see Valgrind configuration.",source:"@site/docs/manual/miscellaneous/92-archived/04-tutorial-pure-dart.md",sourceDirName:"manual/miscellaneous/92-archived",slug:"/manual/miscellaneous/archived/tutorial-pure-dart",permalink:"/flutter_rust_bridge/manual/miscellaneous/archived/tutorial-pure-dart",draft:!1,editUrl:"https://github.com/fzyzcjy/flutter_rust_bridge/tree/master/website/docs/manual/miscellaneous/92-archived/04-tutorial-pure-dart.md",tags:[],version:"current",sidebarPosition:4,frontMatter:{},sidebar:"tutorialSidebar",previous:{title:"Set up Flutter/Dart+Rust support from scratch",permalink:"/flutter_rust_bridge/manual/miscellaneous/archived/set-up-from-scratch"},next:{title:"Misc operations in contributing",permalink:"/flutter_rust_bridge/manual/miscellaneous/archived/misc-contributing"}},u={},s=[{value:"Get example code",id:"get-example-code",level:2},{value:"(Optional) Manually run code generator",id:"optional-manually-run-code-generator",level:2},{value:"Run &quot;Dart+Rust&quot; app",id:"run-dartrust-app",level:2}],p={toc:s};function d(e){let{components:t,...r}=e;return(0,n.kt)("wrapper",(0,a.Z)({},p,r,{components:t,mdxType:"MDXLayout"}),(0,n.kt)("h1",{id:"tutorial-pure-dart"},"Tutorial: Pure Dart"),(0,n.kt)("p",null,(0,n.kt)("strong",{parentName:"p"},"Remark"),": The ",(0,n.kt)("inlineCode",{parentName:"p"},"valgrind_test")," section of the ",(0,n.kt)("a",{parentName:"p",href:"https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/.github/workflows/test.yaml"},"CI workflow")," can also be useful, if you want details of each command and want to see Valgrind configuration."),(0,n.kt)("p",null,"Unlike the previous tutorial, this one integrates Rust with pure Dart instead of Flutter."),(0,n.kt)("h2",{id:"get-example-code"},"Get example code"),(0,n.kt)("p",null,"Please ",(0,n.kt)("a",{parentName:"p",href:"https://dart.dev/get-dart"},"install Dart"),", ",(0,n.kt)("a",{parentName:"p",href:"https://www.rust-lang.org/learn/get-started"},"install Rust"),", and have some familiarity with them. Then run ",(0,n.kt)("inlineCode",{parentName:"p"},"git clone https://github.com/fzyzcjy/flutter_rust_bridge"),", and my example is in ",(0,n.kt)("inlineCode",{parentName:"p"},"frb_example/pure_dart"),"."),(0,n.kt)("h2",{id:"optional-manually-run-code-generator"},"(Optional) Manually run code generator"),(0,n.kt)("p",null,"Remark: Bridge is automatically generated upon running ",(0,n.kt)("inlineCode",{parentName:"p"},"cargo build")," using build-script in build.rs file, so this step is optional. Even if you do it, you should not see anything changed."),(0,n.kt)("p",null,"Install it: ",(0,n.kt)("inlineCode",{parentName:"p"},"cargo install flutter_rust_bridge_codegen"),"."),(0,n.kt)("p",null,"Run it: ",(0,n.kt)("inlineCode",{parentName:"p"},"flutter_rust_bridge_codegen --rust-input frb_example/pure_dart/rust/src/api.rs --dart-output frb_example/pure_dart/dart/lib/bridge_generated.dart")," (See ",(0,n.kt)("a",{parentName:"p",href:"https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/.github/workflows/codegen.yml"},"CI workflow")," as a reference.) (For Windows, you may need ",(0,n.kt)("inlineCode",{parentName:"p"},"\\\\")," instead of ",(0,n.kt)("inlineCode",{parentName:"p"},"/")," for paths.)"),(0,n.kt)("h2",{id:"run-dartrust-app"},'Run "Dart+Rust" app'),(0,n.kt)("p",null,"You may run ",(0,n.kt)("inlineCode",{parentName:"p"},"frb_example/pure_dart/dart/lib/main.dart")," as a normal Dart program, except that you should provide the dynamic linked library of the Rust code (for simplicity, here I only demonstrate the approach for dynamic linked library, but you can for sure use other methods). The detailed steps are as follows."),(0,n.kt)("p",null,"Run ",(0,n.kt)("inlineCode",{parentName:"p"},"cargo build")," in ",(0,n.kt)("inlineCode",{parentName:"p"},"frb_example/pure_dart/rust")," to build the Rust code into a ",(0,n.kt)("inlineCode",{parentName:"p"},".so")," file. Then run ",(0,n.kt)("inlineCode",{parentName:"p"},"dart frb_example/pure_dart/dart/lib/main.dart frb_example/pure_dart/rust/target/debug/libflutter_rust_bridge_example_pure_dart.so")," to run the Dart program with Rust ",(0,n.kt)("inlineCode",{parentName:"p"},".so"),' file. (If you have problems, see "Troubleshooting" section.)  (If on MacOS, Rust may indeed generate ',(0,n.kt)("inlineCode",{parentName:"p"},".dylib"),", so change the last command to use ",(0,n.kt)("inlineCode",{parentName:"p"},"...dylib")," instead of ",(0,n.kt)("inlineCode",{parentName:"p"},"...so"),",)"),(0,n.kt)("p",null,"P.S. You will only see some tests passing - no fancy UI or functionality in this example."))}d.isMDXComponent=!0}}]);