

document.querySelector("#btn_log").onclick = function(event){
    event.preventDefault();
    
    let login = document.querySelector("#login").value;
    let pass = document.querySelector("#pass").value;
   

       
        console.log(login)

        let data = {
            
            "email":login,
            "password": pass,

        }
      
            ajax("https://e28b-188-130-155-167.ngrok.io/api/user/login", "POST", response_reg, JSON.stringify(data));
            function response_reg(dataArr){
               
                    localStorage.setItem("user_data",JSON.stringify(dataArr));
                    window.location.href="../templates/page_of_user.html"
               
                // console.log(dataArr);
            // dataArr = response;

            }
        
        
    
    
}

