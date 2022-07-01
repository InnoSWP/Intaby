<<<<<<< HEAD
=======

>>>>>>> 2dc3a13b95428debac8acab6e51880a37d3f4f01

function addParticipant(number) {
    var wrapper = document.getElementsByClassName('waiting_hall_wrapper')[0]
    // console.log(wrapper)
    for (let index = 0; index < number; index++) {
        var element = createParticipant()
        wrapper.appendChild(element)
    }
}

function createParticipant() {
   var element = document.createElement('div')
   element.classList.add("person",  "mt-3", "col-6", "col-md-2" )
   element.textContent = "Person"
   return element
}