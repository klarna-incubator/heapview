import{c as m,a as n}from"./common/_commonjsHelpers-38687f85.js";var o=m(function(l,p){(function(j,k){l.exports=k()})(n,function(){return j.importState=function(a){var b=new j();return b.importState(a),b},j;function j(){return function(a){var b=0,c=0,d=0,e=1;a.length==0&&(a=[+new Date()]);var g=k();b=g(" "),c=g(" "),d=g(" ");for(var i=0;i<a.length;i++)b-=g(a[i]),b<0&&(b+=1),c-=g(a[i]),c<0&&(c+=1),d-=g(a[i]),d<0&&(d+=1);g=null;var f=function(){var h=2091639*b+e*23283064365386963e-26;return b=c,c=d,d=h-(e=h|0)};return f.uint32=function(){return f()*4294967296},f.fract53=function(){return f()+(f()*2097152|0)*11102230246251565e-32},f.version="Alea 0.9",f.args=a,f.exportState=function(){return[b,c,d,e]},f.importState=function(h){b=+h[0]||0,c=+h[1]||0,d=+h[2]||0,e=+h[3]||0},f}(Array.prototype.slice.call(arguments))}function k(){var a=4022871197,b=function(c){c=c.toString();for(var d=0;d<c.length;d++){a+=c.charCodeAt(d);var e=.02519603282416938*a;a=e>>>0,e-=a,e*=a,a=e>>>0,e-=a,a+=e*4294967296}return(a>>>0)*23283064365386963e-26};return b.version="Mash 0.9",b}})});export default o;