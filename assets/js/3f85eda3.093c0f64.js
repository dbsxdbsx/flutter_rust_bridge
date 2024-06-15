"use strict";(self.webpackChunkflutter_rust_bridge=self.webpackChunkflutter_rust_bridge||[]).push([[1559],{3369:(e,t,n)=>{n.r(t),n.d(t,{default:()=>s});var l=n(7294),r=n(8585),a=n(9996);function c(){return l.createElement("div",null,l.createElement(o,{color:"#4caf50"},"1. Simple"),l.createElement("p",{style:{margin:"4px 0px"}},"Simple Rust..."),l.createElement(a.Z,{language:"rust"},"fn f(a: String, b: Vec<String>) -> MyStruct { ... }"),l.createElement("p",{style:{margin:"4px",marginTop:"-12px",marginLeft:"0px"}},"...called from Dart, without manual intervention."),l.createElement(a.Z,{language:"dart"},"print(f(a: 'Hello', b: ['World']));"),l.createElement(o,{color:"#2196f3"},"2. Fancy"),l.createElement("p",{style:{margin:"4px 0px"}},"Let's see how fancy we can support:"),l.createElement(a.Z,{language:"rust"}," ".repeat(0),l.createElement(m,{color:"#388e3c"},"Arbitrarily fancy Rust types"),"\n",l.createElement("b",null,"struct")," Garden ",l.createElement(u,{color:"#CBF6D3"},"{ land: whatever::fancy::Land }"),"\n\n"," ".repeat(0),l.createElement(m,{color:"#1976d2"},"Complex but auto-translatable"),"\n",l.createElement("b",null,"enum")," Tree ",l.createElement(u,{color:"#cfeeff"},"{ A { name: (String, i32), children: Option<Vec<Tree>> }, B }"),"\n\n"," ".repeat(0),l.createElement(m,{color:"#e65100"},"Support functions & methods"),"\n",l.createElement(u,{color:"#ffe0b2"},l.createElement("b",null,"impl"))," Garden {\n"," ".repeat(4),l.createElement(m,{color:"#c2185b"},"Allow async & sync Rust"),"\n    ",l.createElement(u,{color:"#ffe0eb"},l.createElement("b",null,"async"))," ",l.createElement("b",null,"fn")," plant(\n"," ".repeat(8),l.createElement(m,{color:"#fbc02d"},"Support T/&T/&mut T"),"\n        ",l.createElement(u,{color:"#fff9c4"},l.createElement("b",null,"&mut"))," self,\n        tree: Tree,\n"," ".repeat(8),l.createElement(m,{color:"#689f38"},"Rust can also call Dart"),"\n        chooser: ",l.createElement(u,{color:"#dcedc8"},l.createElement("b",null,"impl Fn"),"(String) -> bool"),",\n"," ".repeat(8),l.createElement(m,{color:"#0288d1"},"Error translation ; zero copy"),"\n    ) -> ",l.createElement(u,{color:"#b2ebf2"},"Result"),"<",l.createElement(u,{color:"#b2ebf2"},"Vec<u8>"),", FancyError> {\n        ...\n    }\n}"),l.createElement("p",{style:{margin:"4px",marginTop:"-12px",marginLeft:"0px"}},"Still seamlessly call in Dart:"),l.createElement(a.Z,null,l.createElement("b",null,"var")," tree = Tree.a(('x', 42), [Tree.b()]);\n"," ".repeat(0),l.createElement(m,{color:"#7b1fa2"},"Async & sync Dart"),"\n",l.createElement("b",null,"print"),"(",l.createElement(u,{color:"#fcd2ff"},l.createElement("b",null,"await"))," garden.plant(tree, (a) => ",l.createElement("b",null,"true"),"));"))}const o=e=>{let{children:t,color:n}=e;return l.createElement("h2",null,t)},m=e=>{let{children:t,color:n}=e;return l.createElement("span",{style:{color:"#888"}},"// ",l.createElement("b",null,l.createElement("span",{style:{color:n}},"\u21b1"))," ",t)},u=e=>{let{children:t,color:n,...r}=e;return l.createElement(l.Fragment,null,l.createElement("span",{style:{background:n}},t))};class s extends l.Component{componentDidMount(){setInterval((()=>{document.body.style.overflow="hidden"}),1e3)}render(){return l.createElement("div",{className:"my-hide-nav"},l.createElement(r.Z,null,l.createElement("div",{style:{padding:"16px",scale:"4.0",transformOrigin:"top left"}},l.createElement("div",{style:{width:"650px"}},l.createElement(c,null)))))}}}}]);