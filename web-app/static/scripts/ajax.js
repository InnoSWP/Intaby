function ajax(url, method, functionName, dataArray){
    let xhttp = new XMLHttpRequest();
    xhttp.open(method, url, true);
    xhttp.setRequestHeader("Content-type", "application/x-www-form-urlencoded");
    xhttp.send(requestData(dataArray));
    xhttp.onreadystatechange = function(){
        if (this.readyState == 4 && this.status==201){
            functionName(this.response);

            
        }
    }
}
function requestData(dataArray){
    let out='';
    for (let key in dataArray){
        out+= `${key}=${dataArray[key]}&`;

    }
    console.log(out);
    return out;
}