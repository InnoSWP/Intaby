let dataArr;
document.querySelector("#btn_log").onclick = function(event){
    event.preventDefault();
   
    let login = document.querySelector("#login").value;
    let pass = document.querySelector("#pass").value;
    const requestURL = 'https://268b-178-205-186-218.ngrok.io/api/user/';
    async function sendRequest(method, url,body = null){
        const headers = {
            //  'Content-Type':'application/json',
            "Origin":"https://javascript.info/"
        }
        const response = await fetch(url, {
            method: method,
            //  body: JSON.stringify(body),
            headers: headers
        });
        // console.log(response.json())
        // console.log(body)
        if (response.status == 200) {
            console.log("Успешный вход");
            console.log(response);
            dataArr = response.json();
            localStorage.setItem("user_data", JSON.stringify(dataArr));
            console.log(localStorage.getItem("user_data"));
        }
        else {
            console.log(response.status);
        }
}

const data = {
"email": login,
"password":pass
}
console.log(JSON.stringify(data))

sendRequest("GET", requestURL, JSON.stringify(data))

}


// document.querySelector("#btn_log").onclick = function(event){
//     event.preventDefault();
    
//     let login = document.querySelector("#login").value;
//     let pass = document.querySelector("#pass").value;
   

       
//         console.log(login)

//         let data = {
            
//             "email":login,
//             "password": pass,

//         }
      
//             ajax("https://268b-178-205-186-218.ngrok.io/api/user", "GET", response_reg, data);
//             function response_reg(dataArr){
//                 // window.location.href="../templates/authorization_page.html"
//             }
        
        
    
    
// }

