let dataArr;
document.querySelector("#btn_log").onclick = function(event){
    event.preventDefault();
   
    let login = document.querySelector("#login").value;
    let pass = document.querySelector("#pass").value;
    const requestURL = 'https://jsonplaceholder.typicode.com/users';
    function sendRequest(method, url,body = null){
        const headers = {
            'Content-Type':'application/json'
        }
        return fetch(url,{
        method:method,
        //  body: JSON.stringify(body),
        headers: headers
    

    }).then(response=>{
        // console.log(response.json())
        // console.log(body)
      if (response.status==200){
            console.log("Успешный вход")
            dataArr = response.json();
            window.location.href="../templates/page_of_user.html";

        }
    })
}

const data = {
"login": login,
"pass":pass
}

sendRequest("GET", requestURL, data)

}
