$(document).ready(function() {

    ($("#connect")).click(function(event){
        event.preventDefault();
       if ($("#name").val() == "" || $("#code").val() == ""){
        alert("Enter your name and code");
       }
       else{
        localStorage.setItem("name", $("#name").val())
        
        let code =  $("#code").val();
        localStorage.setItem("code_user", code);

        data =  ($("#name").val()).toString();

        if (code && data){
  
            ajax(`http://51.250.100.243:8080/games/${code}`, "POST", response_fun, data);
            function response_fun(data_arr){
                // console.log(localStorage.getItem("code"))
                // console.log(12234)
                window.location.href = "../templates/waiting_hall.html"
            }
        }
      

       }
    })
});