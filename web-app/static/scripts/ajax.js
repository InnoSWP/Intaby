function ajax(url, method, functionName, dataArray){
    let xhttp = new XMLHttpRequest();
    xhttp.open(method, url, true);
    xhttp.setRequestHeader("Content-type", "application/json");
    // console.log(dataArray)
    xhttp.send(dataArray);
    xhttp.onreadystatechange = function(){
        if (this.readyState == 4 && (this.status == 200 || this.status == 201)){
            functionName(this.response);
        }
        else if (this.status == 403){
            alert("User already exist")
        }
        
    }
}
