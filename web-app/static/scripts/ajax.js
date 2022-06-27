function ajax(url, method, functionName, dataArray){
    let xhttp = new XMLHttpRequest();
    xhttp.open(method, url, true);
    xhttp.setRequestHeader("Content-type", "application/json");
    console.log(dataArray)
    xhttp.send(dataArray);
    xhttp.onreadystatechange = function(){
        functionName(this.response);
    }
}
