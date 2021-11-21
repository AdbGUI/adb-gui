import { useEffect, useState } from 'react';
import './Fade.css';

const Fade = ({ content, timeWait }) => {
    const [_content, setContent] = useState(content);
    const [fadeProp, setFadeProp] = useState("")

    useEffect(() => {
        if (_content !== content) {
            const timeout = setInterval(() => {
                if (fadeProp === 'fade-in')
                    setFadeProp('fade-out')
                else {
                    setContent(content)
                    setFadeProp('fade-in')
                }
            }, timeWait);
            return () => clearInterval(timeout)
        }
    }, [fadeProp, _content, content, timeWait])
    return (
        <div className={`fade ${fadeProp}`} style={{ transition: `opacity ${timeWait} ease-in-out` }}>
            {_content}
        </div>
    )
}

export default Fade;
