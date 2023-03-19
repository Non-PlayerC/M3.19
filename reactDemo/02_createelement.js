console.log("eiffel is running ..");

// 构建返回的对象
function createElement(type, props, ...children) {
    return {
        type,
        props: {
            ...props,
            children: children.map(child =>
                typeof child == "object" ? child : createTextElement(child)
            )
        }
    }
}
// 创建对应类型的 child 
function createTextElement(text) {
    return {
        type: "TEXT_ELEMENT"
    }
}