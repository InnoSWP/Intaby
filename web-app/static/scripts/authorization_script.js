
// let dataArr;
// const requestURL = 'https://d904-188-130-155-167.ngrok.io/api/user/login';
// // const data1 = {
// //     "email": "cringe",
// //     "password":"123"
// //     }
// document.querySelector("#btn_log").onclick = function(event){
//     event.preventDefault();
   
//     let login = document.querySelector("#login").value;
//     let pass = document.querySelector("#pass").value;
//     async function sendRequest(method, url,body = null){
//         const headers = {
//              'Content-Type':'application/json',
//             "Origin":"https://javascript.info/"
//         }
//         const response = await fetch(url, {
//             method: method,
//             body: JSON.stringify(body),
//             headers: headers
//         });
//         // console.log(response.json())
//         // console.log(body)
//         if (response.status == 200) {
//             console.log("Успешный вход");
//             console.log(response.json());
//             // dataArr = response;

//             localStorage.setItem("user_data",JSON.stringify(dataArr));
//             // window.location.href = "../templates/page_of_user.html"



//         }
//         else {
//             console.log(response.status);
//         }
// }
// const data = {
//     "email": login,
//     "password":pass
//     }

// console.log(data)

// sendRequest("POST", requestURL, data)

// }


document.querySelector("#btn_log").onclick = function(event){
    event.preventDefault();
    
    let login = document.querySelector("#login").value;
    let pass = document.querySelector("#pass").value;
   

       
        console.log(login)

        let data = {
            
            "email":login,
            "password": pass,

        }
      
            ajax("https://f865-188-130-155-167.ngrok.io/api/user/login", "POST", response_reg, JSON.stringify(data));
            function response_reg(dataArr){
               
                    localStorage.setItem("user_data",JSON.stringify(dataArr));
                    window.location.href="../templates/page_of_user.html"
               
                // console.log(dataArr);
            // dataArr = response;

            }
        
        
    
    
}

