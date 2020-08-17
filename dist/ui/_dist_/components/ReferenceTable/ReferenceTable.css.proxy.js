const code=`.reference tr {
  line-height: 1.4rem;
}

.reference .colored-square {
  display: block;
  width: 1rem;
  height: 1rem;
  margin: auto 0.2rem;
  border: 1px solid #ccc;
}

.reference .total {
  font-weight: bold;
}

.reference .size {
  text-align: right;
}
`,styleEl=document.createElement("style"),codeEl=document.createTextNode(code);styleEl.type="text/css",styleEl.appendChild(codeEl),document.head.appendChild(styleEl);
