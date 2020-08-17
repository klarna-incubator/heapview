const code=`.chart.container {
  display: flex;
}

.chart.container > div {
  margin: 1rem;
}
`,styleEl=document.createElement("style"),codeEl=document.createTextNode(code);styleEl.type="text/css",styleEl.appendChild(codeEl),document.head.appendChild(styleEl);
