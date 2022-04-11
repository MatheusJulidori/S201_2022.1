function arrayCreator(array){
    for (var i=1; i<=10; i++)
        array.push(i)
    return array
}

const array = []

console.log(arrayCreator(array))