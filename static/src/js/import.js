let importBtn = document.getElementById('btn-import-feed-url');
console.log(importBtn);
importBtn.addEventListener('click', function(){
    let feedURL = document.getElementById('import-feed-url').value;
    let oReq = new XMLHttpRequest();
    oReq.addEventListener('load', (e)=>{
        console.log('Req completed', e);
    });
    oReq.open('GET', '/api/import/url/' + encodeURIComponent(feedURL));
    oReq.send();
});