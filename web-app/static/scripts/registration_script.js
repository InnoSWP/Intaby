document.querySelector("#btn_reg").onclick = function(event){
    event.preventDefault();
    
    let name = document.querySelector("#signup_name").value;
    let email = document.querySelector("#signup_email").value;
    let surname = document.querySelector("#signup_surname").value;
    let pass = document.querySelector("#signup_pass").value;
    let pass2 = document.querySelector("#signup_pass2").value;
    if (pass != pass2){
        alert("Password do not match, try again")
    }
    else{

       
        console.log(name)

        let data = {
            "name": name,
            "surname":surname,
            "email":email,
            "password": pass,

        }
        if (name&&surname&&pass&&email&&pass2){
            ajax("http://localhost:8888/api/user/register", "POST", response_reg, JSON.stringify(data));
            function response_reg(dataArr){
                
                    window.location.href="/authorization_page.html"
                
               
                
            }
        
        }
     
    }
    
}
