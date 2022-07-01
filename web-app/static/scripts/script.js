$(document).ready(function() {
   
    ($("#connect_")).click(function(event){
        event.preventDefault();
       if ($("#name").val() == ""){
        alert("Enter your name");
       }
       else{
        localStorage.setItem("name",$("#name").val())
        localStorage.setItem("code",$("#code").val())
        data = {
            "name": $("#name").val(),
            "code": $("#code").val()
        }
        ajax("", "POST", response_fun, data)
        function response_fun(data_arr){
            // console.log(localStorage.getItem("code"))
            // console.log(12234)
            window.location.href = "../templates/waiting_hall.html"
        }
        
       }
    })
});
