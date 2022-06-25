function ajax(url, method, functionName, dataArray){
    let xhttp = new XMLHttpRequest();
    xhttp.open(method, url, true);
    xhttp.setRequestHeader("Content-type", "application/json");
    console.log(JSON.stringify(dataArray))
    xhttp.send(JSON.stringify(dataArray));
    xhttp.onreadystatechange = function(){
        if (this.readyState == 4 && this.status==201){
            functionName(this.response);

        }
    }
}
function requestData(dataArray){
    console.log(JSON.stringify(dataArray));
    return JSON.stringify(dataArray);
}