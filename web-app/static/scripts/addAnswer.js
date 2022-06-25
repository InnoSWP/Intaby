// Add answer

var addButton = document.getElementById("add_button")
var clicks = 2
var wrapper = document.getElementsByClassName("Answer_add_wrapper")

addButton.addEventListener('click', function addAnswer() {
    clicks += 1
    if (clicks <= 6) {
        var newAnswer = document.createElement('div')
        newAnswer.classList.add('answer_wrapper', 'd-flex', 'col-12', 'position-relative')
        var textField = document.createElement('textarea')
        textField.classList.add('Answer_type', 'text-white', 'py-2', 'mb-2', 'pe-5', 'ps-2', 'mb-lg-4', 'col-12', 'rounded')
        textField.placeholder = "Answer " + clicks
        newAnswer.append(textField)
        if (answerDropList.options[answerDropList.selectedIndex].text === "Multiple answer") {
            let badge = document.createElement('button')
            badge.classList.add('Badge', 'position-absolute', 'top-0', 'end-0', 'p-2', 'mt-2', 'me-2', 'border', 'border-light', 'rounded-2', 'border-5')
            badge.style.color = 'white'
            newAnswer.append(badge)
        }
        if (answerDropList.options[answerDropList.selectedIndex].text === "Single answer") {
            let badge = document.createElement('button')
            badge.classList.add('Badge', 'position-absolute', 'top-0', 'end-0', 'p-2', 'mt-2', 'me-2', 'border', 'border-light', 'rounded-circle', 'border-5')
            badge.style.color = 'white'
            newAnswer.append(badge)
        }
        if (clicks === 3) {
            textField.style.backgroundColor = "#FF943B";
        }
        if (clicks === 4) {
            textField.style.backgroundColor = "#467CD3";
        }
        if (clicks === 5) {
            textField.style.backgroundColor = "#8AC600";
        }
        if (clicks === 6) {
            textField.style.backgroundColor = "#9D3FE1";
        }
        if (clicks < 6) {
            wrapper[0].append(newAnswer)
            wrapper[0].append(addButton)
        }
        else {
            wrapper[0].append(newAnswer)
            addButton.parentNode.removeChild(addButton)
        }
    }
    addEvents()
})

// Delete answer

var deleteButton = document.getElementsByClassName("delete_answer")

deleteButton[0].addEventListener('click', function DeleteAnswer() {
    if (clicks === 6) {
        wrapper[0].removeChild(wrapper[0].lastElementChild)
        wrapper[0].append(addButton)
        clicks -= 1
    }
    else if (clicks > 2) {
        var storage = wrapper[0].lastElementChild
        addButton.parentNode.removeChild(addButton)
        wrapper[0].removeChild(wrapper[0].lastElementChild)
        wrapper[0].append(storage)
        clicks -= 1
    }
})

// Select type of answer

var answerDropList = document.getElementById("answerType")
var singleQuestion = document.getElementById("singleAnswer")

answerDropList.addEventListener('change', function ReceiveType() {
    var answerType = answerDropList.options[answerDropList.selectedIndex].text
    var answerButtons = document.getElementsByClassName('answer_wrapper')
    switch (answerType) {
        case 'Multiple answer':
            clearSelectedAnswers()
            //console.log(1)
            // console.log(answerButtons)
            //console.log(document.getElementsByClassName('Badge')[0])
            if (document.getElementsByClassName('Badge')[0] === undefined) {
                for (let item of answerButtons) {
                    let badge = document.createElement('button')
                    badge.classList.add('Badge', 'position-absolute', 'top-0', 'end-0', 'p-2', 'mt-2', 'me-2', 'border', 'border-light', 'rounded-2', 'border-5')
                    badge.style.color = 'white'
                    item.append(badge)
                }
            }
            else {
                let badges = document.getElementsByClassName('Badge')
                for (let item of badges) {
                    //console.log(item)
                    item.classList.remove('rounded-circle')
                    item.classList.add('rounded-2')
                }
            }
            break
        case 'Single answer':
            clearSelectedAnswers()
            if (document.getElementsByClassName('Badge')[0] === undefined) {
                for (let item of answerButtons) {
                    let badge = document.createElement('button')
                    badge.classList.add('Badge', 'position-absolute', 'top-0', 'end-0', 'p-2', 'mt-2', 'me-2', 'border', 'border-light', 'rounded-circle', 'border-5')
                    badge.style.color = 'white'
                    item.append(badge)
                }
            }
            else {
                let badges = document.getElementsByClassName('Badge')
                for (let item of badges) {
                    //console.log(item)
                    item.classList.remove('rounded-2')
                    item.classList.add('rounded-circle')
                }
            }
            break
        case 'Interview':
            if (document.getElementsByClassName('Badge')[0] !== undefined) {
                for (let item of answerButtons) {
                    let badge = document.getElementsByClassName('Badge')
                    badge[0].parentNode.removeChild(badge[0])
                }
            }
            break
    }
    addEvents()
})

// Select answer 
var badgesButtons
var ifSelected = false
var selectedAnswer

function addEvents() {
    badgesButtons = document.getElementsByClassName('Badge')
    for (let item of badgesButtons) {
        if (item.getAttribute('listener') !== 'true') {
            item.setAttribute('listener', 'true');
            item.addEventListener('click', () => {
                if (answerDropList.options[answerDropList.selectedIndex].text === "Multiple answer") {
                    item.parentNode.lastChild.classList.toggle('bg-dark')
                }
                if (answerDropList.options[answerDropList.selectedIndex].text === "Single answer") {
                    if (ifSelected === true) {
                        selectedAnswer.parentNode.lastChild.classList.toggle('bg-dark')
                        item.parentNode.lastChild.classList.toggle('bg-dark')
                    }
                    else {
                        item.parentNode.lastChild.classList.toggle('bg-dark')
                    }
                    selectedAnswer = item
                    ifSelected = true
                }
            })
        }
    }
}

function clearSelectedAnswers() {
    badgesButtons = document.getElementsByClassName('Badge')
    ifSelected = false
    for (let item of badgesButtons) {
        if (item.parentNode.lastChild.classList.contains('bg-dark')) {
            item.parentNode.lastChild.classList.toggle('bg-dark')
        }
    }
}