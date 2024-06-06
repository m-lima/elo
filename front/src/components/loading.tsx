import './loading.css';

const Paddle = () => {
  return (
    <svg viewBox='0 0 221.98541 221.98542' xmlns='http://www.w3.org/2000/svg'>
      <path
        style='fill:#ffa500'
        d='m 75.753165,86.016683 c 0,0 -0.2787,-10.06525 26.995015,-17.28915 33.62139,-8.905204 72.02012,-4.94975 91.94941,4.3327 19.9293,9.28245 28.81047,17.68222 27.05527,29.269747 -1.77957,11.20461 0.0251,-0.13288 -1.34794,7.12488 0,0 -2.60694,21.62977 -53.72542,28.01809 -20.01427,2.5012 -38.99426,-4.71782 -38.99426,-4.71782 L 75.851245,96.300293 Z'
      />
      <path
        style='fill:#bbbbbb'
        d='m 1.2513869,126.08261 c 0,0 71.3984281,-22.59675 74.3430481,-23.6007 2.94462,-1.00395 1.46766,-10.270367 1.46766,-10.270367 l 51.196015,33.355817 v 4.96978 c 0,0 -24.98707,-6.84741 -29.417913,-5.61104 -28.385622,11.06353 -57.143115,22.36264 -85.592656,33.23793 z'
      />
      <path
        id='paddle-butt'
        style='fill:#999999'
        d='m 2.6420359,126.82952 c 6.2503392,6.84925 2.745432,2.67014 7.1895022,7.98833 4.4440699,5.3182 4.3700899,12.78134 4.3700899,12.78134 l 0.140971,5.77979 c 0,0 0.280557,7.9228 -3.289315,3.57126 -3.5698709,-4.35154 -3.5260489,-4.34016 -7.0485321,-8.74018 -3.522483,-4.40002 -3.759218,-12.12347 -3.759218,-12.12347 l -0.234951,-4.51107 c 0,0 -0.325321,-7.98609 2.631453,-4.746 z'
      />
      <circle id='paddle-ball' style='fill:#ffffff' cx='137' cy='86' r='14' />
      <text x='80' y='180' style='fill:#ffa500'>
        Loading
      </text>
      <circle id='paddle-dot-1' style='fill:#ffa500' cx='137' cy='180' r='1' />
      <circle id='paddle-dot-2' style='fill:#ffa500' cx='140' cy='180' r='1' />
      <circle id='paddle-dot-3' style='fill:#ffa500' cx='143' cy='180' r='1' />
    </svg>
  );
};

export const Loading = () => {
  return (
    <div class='components_loading'>
      <Paddle />
    </div>
  );
};
