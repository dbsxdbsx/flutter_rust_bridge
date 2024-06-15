"use strict";(self.webpackChunkflutter_rust_bridge=self.webpackChunkflutter_rust_bridge||[]).push([[766],{3905:(e,t,r)=>{r.d(t,{Zo:()=>u,kt:()=>y});var n=r(7294);function a(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function i(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,n)}return r}function l(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?i(Object(r),!0).forEach((function(t){a(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):i(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function s(e,t){if(null==e)return{};var r,n,a=function(e,t){if(null==e)return{};var r,n,a={},i=Object.keys(e);for(n=0;n<i.length;n++)r=i[n],t.indexOf(r)>=0||(a[r]=e[r]);return a}(e,t);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(n=0;n<i.length;n++)r=i[n],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(a[r]=e[r])}return a}var o=n.createContext({}),p=function(e){var t=n.useContext(o),r=t;return e&&(r="function"==typeof e?e(t):l(l({},t),e)),r},u=function(e){var t=p(e.components);return n.createElement(o.Provider,{value:t},e.children)},c={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},d=n.forwardRef((function(e,t){var r=e.components,a=e.mdxType,i=e.originalType,o=e.parentName,u=s(e,["components","mdxType","originalType","parentName"]),d=p(r),y=a,m=d["".concat(o,".").concat(y)]||d[y]||c[y]||i;return r?n.createElement(m,l(l({ref:t},u),{},{components:r})):n.createElement(m,l({ref:t},u))}));function y(e,t){var r=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var i=r.length,l=new Array(i);l[0]=d;var s={};for(var o in t)hasOwnProperty.call(t,o)&&(s[o]=t[o]);s.originalType=e,s.mdxType="string"==typeof e?e:a,l[1]=s;for(var p=2;p<i;p++)l[p]=r[p];return n.createElement.apply(null,l)}return n.createElement.apply(null,r)}d.displayName="MDXCreateElement"},6384:(e,t,r)=>{r.r(t),r.d(t,{assets:()=>o,contentTitle:()=>l,default:()=>c,frontMatter:()=>i,metadata:()=>s,toc:()=>p});var n=r(7462),a=(r(7294),r(3905));const i={},l="Type alias",s={unversionedId:"guides/types/translatable/detailed/alias",id:"guides/types/translatable/detailed/alias",title:"Type alias",description:"Type alias is also supported. For example:",source:"@site/docs/guides/types/translatable/detailed/alias.md",sourceDirName:"guides/types/translatable/detailed",slug:"/guides/types/translatable/detailed/alias",permalink:"/flutter_rust_bridge/guides/types/translatable/detailed/alias",draft:!1,editUrl:"https://github.com/fzyzcjy/flutter_rust_bridge/tree/master/website/docs/guides/types/translatable/detailed/alias.md",tags:[],version:"current",frontMatter:{},sidebar:"tutorialSidebar",previous:{title:"Options",permalink:"/flutter_rust_bridge/guides/types/translatable/detailed/option"},next:{title:"Map and Set",permalink:"/flutter_rust_bridge/guides/types/translatable/detailed/map_set"}},o={},p=[{value:"Limitation",id:"limitation",level:2}],u={toc:p};function c(e){let{components:t,...r}=e;return(0,a.kt)("wrapper",(0,n.Z)({},u,r,{components:t,mdxType:"MDXLayout"}),(0,a.kt)("h1",{id:"type-alias"},"Type alias"),(0,a.kt)("p",null,"Type alias is also supported. For example:"),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},"enum MyEnum {...}\nstruct MyStruct {...}\n\n// type aliases\npub type Id = u64;\npub type EnumAlias = MyEnum;\npub type StructAlias = MyStruct;\n\n// can also use them in fields, etc\npub struct TestModel { pub id: Id, pub e: EnumAlias, pub s: StructAlias}\n\npub fn f(input: Id) -> TestModel {...}\n")),(0,a.kt)("h2",{id:"limitation"},"Limitation"),(0,a.kt)("p",null,"The ",(0,a.kt)("inlineCode",{parentName:"p"},"ItemType")," inside Generic is not supported yet, such as ",(0,a.kt)("inlineCode",{parentName:"p"},"SyncReturn<Id>"),". The nested ",(0,a.kt)("inlineCode",{parentName:"p"},"ItemType")," may also not be supported."))}c.isMDXComponent=!0}}]);