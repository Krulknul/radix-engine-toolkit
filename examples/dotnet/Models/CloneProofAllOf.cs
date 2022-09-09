/*
 * Transaction Lib
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://github.com/openapitools/openapi-generator.git
 */

using System;
using System.Linq;
using System.IO;
using System.Text;
using System.Text.RegularExpressions;
using System.Collections;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Runtime.Serialization;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;
using System.ComponentModel.DataAnnotations;

namespace Models
{
    /// <summary>
    /// CloneProofAllOf
    /// </summary>
    [DataContract]
    public partial class CloneProofAllOf :  IEquatable<CloneProofAllOf>, IValidatableObject
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="CloneProofAllOf" /> class.
        /// </summary>
        [JsonConstructorAttribute]
        protected CloneProofAllOf() { }
        /// <summary>
        /// Initializes a new instance of the <see cref="CloneProofAllOf" /> class.
        /// </summary>
        /// <param name="proof">proof (required).</param>
        /// <param name="intoProof">intoProof (required).</param>
        public CloneProofAllOf(Proof proof = default(Proof), Proof intoProof = default(Proof))
        {
            // to ensure "proof" is required (not null)
            if (proof == null)
            {
                throw new InvalidDataException("proof is a required property for CloneProofAllOf and cannot be null");
            }
            else
            {
                this.Proof = proof;
            }

            // to ensure "intoProof" is required (not null)
            if (intoProof == null)
            {
                throw new InvalidDataException("intoProof is a required property for CloneProofAllOf and cannot be null");
            }
            else
            {
                this.IntoProof = intoProof;
            }

        }

        /// <summary>
        /// Gets or Sets Proof
        /// </summary>
        [DataMember(Name="proof", EmitDefaultValue=true)]
        public Proof Proof { get; set; }

        /// <summary>
        /// Gets or Sets IntoProof
        /// </summary>
        [DataMember(Name="into_proof", EmitDefaultValue=true)]
        public Proof IntoProof { get; set; }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            var sb = new StringBuilder();
            sb.Append("class CloneProofAllOf {\n");
            sb.Append("  Proof: ").Append(Proof).Append("\n");
            sb.Append("  IntoProof: ").Append(IntoProof).Append("\n");
            sb.Append("}\n");
            return sb.ToString();
        }

        /// <summary>
        /// Returns the JSON string presentation of the object
        /// </summary>
        /// <returns>JSON string presentation of the object</returns>
        public virtual string ToJson()
        {
            return Newtonsoft.Json.JsonConvert.SerializeObject(this, Newtonsoft.Json.Formatting.Indented);
        }

        /// <summary>
        /// Returns true if objects are equal
        /// </summary>
        /// <param name="input">Object to be compared</param>
        /// <returns>Boolean</returns>
        public override bool Equals(object input)
        {
            return this.Equals(input as CloneProofAllOf);
        }

        /// <summary>
        /// Returns true if CloneProofAllOf instances are equal
        /// </summary>
        /// <param name="input">Instance of CloneProofAllOf to be compared</param>
        /// <returns>Boolean</returns>
        public bool Equals(CloneProofAllOf input)
        {
            if (input == null)
                return false;

            return 
                (
                    this.Proof == input.Proof ||
                    (this.Proof != null &&
                    this.Proof.Equals(input.Proof))
                ) && 
                (
                    this.IntoProof == input.IntoProof ||
                    (this.IntoProof != null &&
                    this.IntoProof.Equals(input.IntoProof))
                );
        }

        /// <summary>
        /// Gets the hash code
        /// </summary>
        /// <returns>Hash code</returns>
        public override int GetHashCode()
        {
            unchecked // Overflow is fine, just wrap
            {
                int hashCode = 41;
                if (this.Proof != null)
                    hashCode = hashCode * 59 + this.Proof.GetHashCode();
                if (this.IntoProof != null)
                    hashCode = hashCode * 59 + this.IntoProof.GetHashCode();
                return hashCode;
            }
        }

        /// <summary>
        /// To validate all properties of the instance
        /// </summary>
        /// <param name="validationContext">Validation context</param>
        /// <returns>Validation Result</returns>
        IEnumerable<System.ComponentModel.DataAnnotations.ValidationResult> IValidatableObject.Validate(ValidationContext validationContext)
        {
            yield break;
        }
    }

}
