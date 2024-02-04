import clsx from "clsx";
import Heading from "@theme/Heading";
import styles from "./styles.module.css";

const FeatureList = [
  {
    title: "Simple setup and integration",
    imageUrl: require("@site/static/img/wizard.png").default,
    description: (
      <>
        {" "}
        Enable your rules using a simple configuration file, and{" "}
        <a href="/docs/integration">
          integrate{" "}
          <span className={styles.text}>
            <span>git-</span>
            <span className={styles.accentText}>sumi</span>
          </span>{" "}
          into your existing workflow
        </a>
        .
      </>
    ),
  },
  {
    title: "Adaptable",
    imageUrl: require("@site/static/img/chameleon.png").default,
    description: (
      <>
        <span className={styles.text}>
          <span>git-</span>
          <span className={styles.accentText}>sumi</span>
        </span>{" "}
        follows the <a href="/docs/rules">rules</a> of each project you work on.
        Use a personal config as a fallback to enforce your own rules.
      </>
    ),
  },
  {
    title: "Non-opinionated",
    imageUrl: require("@site/static/img/sculptor.png").default,
    description: (
      <>
        By default,{" "}
        <span className={styles.text}>
          <span>git-</span>
          <span className={styles.accentText}>sumi</span>
        </span>{" "}
        has no rules enabled. Start from scratch or draw inspiration from{" "}
        <a href="/docs/examples">the examples</a>.
      </>
    ),
  },
];

function Feature({ imageUrl, title, description }) {
  return (
    <div className={clsx("col col--4")}>
      <div className="text--center">
        <img src={imageUrl} className={styles.featureImg} alt={title} />
      </div>
      <div className="text--center padding-horiz--md">
        <Heading as="h3">{title}</Heading>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures() {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
