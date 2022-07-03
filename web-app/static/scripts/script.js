$(document).ready(function() {

    ($("#connect")).click(function(event){
        event.preventDefault();
       if ($("#name").val() == ""){
        alert("Enter your name");
       }
       else{
        localStorage.setItem("name", $("#name").val())
        
        let code =  $("#code").val();
        data =  ($("#name").val()).toString();
        
        ajax(`http://51.250.19.225:8080/games/${code}`, "POST", response_fun,data);
        function response_fun(data_arr){
            // console.log(localStorage.getItem("code"))
            // console.log(12234)
            window.location.href = "/waiting_hall.html"
        }

       }
    })
});