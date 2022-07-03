// Add answer
var answerDropList = document.getElementById("answerType")
var addButton = document.getElementById("add_button")
var clicks = 2
var wrapper = document.getElementsByClassName("Answer_add_wrapper")

sessionStorage.setItem('type', 'Interview')

addButton.addEventListener('click', function addAnswer() {
    clicks += 1
    if (clicks <= 6) {

        var newAnswer = document.createElement('div')
        newAnswer.classList.add('answer_wrapper', 'd-flex', 'col-12', 'position-relative')
        var textField = document.createElement('textarea')
        textField.classList.add('Answer_type', 'text-white', 'py-2', 'mb-2', 'px-5', 'mb-lg-4', 'col-12', 'rounded')
        textField.placeholder = "Answer " + clicks
        var deleteButton = document.createElement('button')
        deleteButton.classList.add('Delete_button', 'position-absolute', 'top-0', 'end-0', 'mt-2', 'p-2')
        newAnswer.append(textField)
        newAnswer.append(deleteButton)

        if (answerDropList.options[answerDropList.selectedIndex].text === "Multiple answer") {
            let badge = document.createElement('button')
            badge.classList.add('Badge', 'position-absolute', 'top-50', 'end-0', 'translate-middle-y', 'p-2', 'mt-2', 'me-2', 'border', 'border-light', 'rounded-2', 'border-5')
            badge.style.color = 'white'
            newAnswer.append(badge)
        }
        if (answerDropList.options[answerDropList.selectedIndex].text === "Single answer") {
            let badge = document.createElement('button')
            badge.classList.add('Badge', 'position-absolute', 'top-50', 'end-0', 'translate-middle-y', 'p-2', 'mt-2', 'me-2', 'border', 'border-light', 'rounded-circle', 'border-5')
            badge.style.color = 'white'
            newAnswer.append(badge)
        }
        if (clicks === 3) {
            textField.style.backgroundColor = "#FF943B";
            // sessionStorage.setItem('color', '#FF943B')
        }
        if (clicks === 4) {
            textField.style.backgroundColor = "#467CD3";
            // sessionStorage.setItem('color', '#467CD3')
        }
        if (clicks === 5) {
            textField.style.backgroundColor = "#8AC600";
            // sessionStorage.setItem('color', '#8AC600')
        }
        if (clicks === 6) {
            textField.style.backgroundColor = "#9D3FE1";
            // sessionStorage.setItem('color', '#9D3FE1')
        }
        if (clicks < 6) {
            wrapper[0].append(newAnswer)
            wrapper[0].append(addButton)
        }
        else {
            wrapper[0].append(newAnswer)
            addButton.parentNode.removeChild(addButton)
        }
        sessionStorage.setItem('number', clicks)
    }
    addEventSelect()
    // addEventDelete()
})


// Delete answer

// var deleteButton = document.getElementsByClassName("delete_answer")

// deleteButton[0].addEventListener('click', function DeleteAnswer() {
//     if (clicks === 6) {
//         wrapper[0].removeChild(wrapper[0].lastElementChild)
//         wrapper[0].append(addButton)
//         clicks -= 1
//     }
//     else if (clicks > 2) {
//         var storage = wrapper[0].lastElementChild
//         addButton.parentNode.removeChild(addButton)
//         wrapper[0].removeChild(wrapper[0].lastElementChild)
//         wrapper[0].append(storage)
//         clicks -= 1
//     }
// })

// Select type of answer

var singleQuestion = document.getElementById("singleAnswer")

answerDropList.addEventListener('change', function ReceiveType() {
    var answerType = answerDropList.options[answerDropList.selectedIndex].text
    var answerButtons = document.getElementsByClassName('answer_wrapper')
    //console.log(answerButtons)
    switch (answerType) {
        case 'Multiple answer':
            clearSelectedAnswers()
            //console.log(1)
            // console.log(answerButtons)
            //console.log(document.getElementsByClassName('Badge')[0])
            if (document.getElementsByClassName('Badge')[0] === undefined) {
                for (let item of answerButtons) {
                    let badge = document.createElement('button')
                    badge.classList.add('Badge', 'position-absolute', 'top-50', 'end-0', 'translate-middle-y', 'p-2', 'mt-2', 'me-2', 'border', 'border-light', 'rounded-2', 'border-5')
                    badge.style.color = 'white'
                    item.append(badge)
                }
                sessionStorage.setItem('type', 'Multiple answer')
            }
            else {
                let badges = document.getElementsByClassName('Badge')
                for (let item of badges) {
                    //console.log(item)
                    item.classList.remove('rounded-circle')
                    item.classList.add('rounded-2')
                }
                sessionStorage.setItem('type', 'Multiple answer')
            }
            break
        case 'Single answer':
            clearSelectedAnswers()
            if (document.getElementsByClassName('Badge')[0] === undefined) {
                for (let item of answerButtons) {
                    let badge = document.createElement('button')
                    badge.classList.add('Badge', 'position-absolute', 'top-50', 'end-0', 'translate-middle-y', 'p-2', 'mt-2', 'me-2', 'border', 'border-light', 'rounded-circle', 'border-5')
                    badge.style.color = 'white'
                    item.append(badge)
                }
                sessionStorage.setItem('type', 'Single answer')
            }
            else {
                let badges = document.getElementsByClassName('Badge')
                for (let item of badges) {
                    //console.log(item)
                    item.classList.remove('rounded-2')
                    item.classList.add('rounded-circle')
                }
                sessionStorage.setItem('type', 'Single answer')
            }
            break
        case 'Interview':
            if (document.getElementsByClassName('Badge')[0] !== undefined) {
                for (let item of answerButtons) {
                    let badge = document.getElementsByClassName('Badge')
                    badge[0].parentNode.removeChild(badge[0])
                }
            }
            sessionStorage.setItem('type', 'Interview')
            break
    }
    addEventSelect()
})

// Select answer 
var badgesButtons
var ifSelected = false
var selectedAnswer

function addEventSelect() {
    badgesButtons = document.getElementsByClassName('Badge')

    for (let item of badgesButtons) {
        if (item.getAttribute('listener') !== 'true') {
            item.setAttribute('listener', 'true')
            item.addEventListener('click', () => {
                if (answerDropList.options[answerDropList.selectedIndex].text === "Multiple answer") {
                    item.parentNode.lastChild.classList.toggle('bg-dark')
                    item.parentNode.classList.toggle('selected')
                }
                if (answerDropList.options[answerDropList.selectedIndex].text === "Single answer") {
                    if (ifSelected === true) {
                        selectedAnswer.parentNode.lastChild.classList.toggle('bg-dark')
                        item.parentNode.lastChild.classList.toggle('bg-dark')
                        item.parentNode.classList.toggle('selected')
                    }
                    else {
                        item.parentNode.lastChild.classList.toggle('bg-dark')
                        item.parentNode.classList.toggle('selected')
                    }
                    selectedAnswer = item
                    ifSelected = true
                }
            })
        }
    }
}

var answers

// function addEventDelete() {
//     answers = document.getElementsByClassName('answer_wrapper')
//     for (let item of answers) {
//         if (item.getAttribute('listener') !== 'true') {
//             item.setAttribute('listener', 'true')
//             let deleteButton = item.getElementsByClassName('Delete_button')
//             deleteButton[0].addEventListener('click', () => {
//                 if (clicks === 6) {
//                     wrapper[0].append(addButton)
//                     deleteButton[0].parentNode.remove()
//                     clicks -= 1
//                     sessionStorage.setItem('number', clicks)
//                     update()
//                 }
//                 else if (clicks > 2) {
//                     deletedN = Number((deleteButton[0].parentNode.firstChild.placeholder)[7])
//                     deleteButton[0].parentNode.remove()
//                     clicks -= 1
//                     sessionStorage.setItem('number', clicks)
//                     update()
//                 }
//             })
//         }
//     }
// }

function update() {
    answers = document.getElementsByClassName('answer_wrapper')
    // console.log(answers)
    counter = 1
    for (let item of answers) {
        item.firstChild.placeholder = "Answer " + counter
        if (counter === 1) {
            item.firstChild.style.backgroundColor = "#F83962";
        }
        if (counter === 2) {
            item.firstChild.style.backgroundColor = "#2EBFD5";
        }
        if (counter === 3) {
            item.firstChild.style.backgroundColor = "#FF943B";
        }
        if (counter === 4) {
            item.firstChild.style.backgroundColor = "#467CD3";
        }
        if (counter === 5) {
            item.firstChild.style.backgroundColor = "#8AC600";
        }
        if (counter === 6) {
            item.firstChild.style.backgroundColor = "#9D3FE1";
        }
        counter += 1
    }

}


function clearSelectedAnswers() {
    badgesButtons = document.getElementsByClassName('Badge')
    ifSelected = false
    for (let item of badgesButtons) {
        if (item.parentNode.lastChild.classList.contains('bg-dark')) {
            item.parentNode.lastChild.classList.toggle('bg-dark')
        }
        if (item.parentNode.classList.contains('selected')) {
            item.parentNode.classList.remove('selected')
        }
    }
}

var addQuestion = document.getElementById("btn_create")

addQuestion.addEventListener('click', () => {
    // clearSelectedAnswers()
})