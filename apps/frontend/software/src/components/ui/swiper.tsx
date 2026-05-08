import { useRef } from "react";
import { useTranslation } from "react-i18next";
import { Swiper, SwiperSlide } from "swiper/react";
import { Pagination, Autoplay } from "swiper/modules";
import { gsap } from "gsap";

import "swiper/css";
import "swiper/css/pagination";

import img1 from "../../assets/1.jpeg";
import img2 from "../../assets/2.jpeg";
import img3 from "../../assets/3.jpeg";
import img4 from "../../assets/4.jpeg";

function HeroSwiper() {
  const { t } = useTranslation();

  const titleRef = useRef(null);
  const textRef = useRef(null);
  const imgRef = useRef(null);

  const animateSlide = () => {
    gsap.fromTo(
      imgRef.current,
      { scale: 1.1, opacity: 0.8 },
      { scale: 1, opacity: 1, duration: 0.8 }
    );

    gsap.fromTo(
      titleRef.current,
      { opacity: 0, y: 20 },
      { opacity: 1, y: 0, duration: 0.6 }
    );

    gsap.fromTo(
      textRef.current,
      { opacity: 0, y: 10 },
      { opacity: 1, y: 0, duration: 0.6, delay: 0.1 }
    );
  };

  const slides = [
    { id: "s1", img: img1 },
    { id: "s2", img: img2 },
    { id: "s3", img: img3 },
    { id: "s4", img: img4 }
  ];

  return (
    <Swiper
      modules={[Pagination, Autoplay]}
      pagination={{ clickable: true }}
      autoplay={{ delay: 8000 }}
      loop
      onSlideChange={animateSlide}
      className="h-dvh w-dvw"
    >
      {slides.map((slide) => (
        <SwiperSlide key={slide.id}>
          <div className="relative h-full w-full flex flex-col items-center justify-center text-white px-6 overflow-hidden">

          
            <img
              ref={imgRef}
              src={slide.img}
              className="absolute inset-0 w-full h-full object-cover"
            />

            <div className="absolute inset-0 bg-black/60" />

            <div className="absolute left-10 xl:left-80  z-10 flex flex-col items-center">

              <h1
                ref={titleRef}
                className="text-3xl font-bold mb-3 text-center"
              >
                {t(`onboarding.slides.${slide.id}.title`)}
              </h1>

              <p
                ref={textRef}
                className="text-gray-300 text-center text-2xl max-w-md"
              >
                {t(`onboarding.slides.${slide.id}.text`)}
              </p>

            </div>
          </div>
        </SwiperSlide>
      ))}
    </Swiper>
  );
}

export default HeroSwiper;