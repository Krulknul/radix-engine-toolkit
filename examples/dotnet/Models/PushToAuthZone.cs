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
    /// PushToAuthZone
    /// </summary>
    [DataContract]
    public partial class PushToAuthZone : Instruction,  IEquatable<PushToAuthZone>, IValidatableObject
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="PushToAuthZone" /> class.
        /// </summary>
        [JsonConstructorAttribute]
        protected PushToAuthZone() { }
        /// <summary>
        /// Initializes a new instance of the <see cref="PushToAuthZone" /> class.
        /// </summary>
        /// <param name="proof">proof (required).</param>
        public PushToAuthZone (Proof proof = default(Proof)) : base ("PUSH_TO_AUTH_ZONE")
        {
            // to ensure "proof" is required (not null)
            if (proof == null)
            {
                throw new InvalidDataException("proof is a required property for PushToAuthZone and cannot be null");
            }
            else
            {
                this.Proof = proof;
            }

        }

        /// <summary>
        /// Gets or Sets Proof
        /// </summary>
        [DataMember(Name="proof", EmitDefaultValue=true)]
        public Proof Proof { get; set; }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            var sb = new StringBuilder();
            sb.Append("class PushToAuthZone {\n");
            sb.Append("  ").Append(base.ToString().Replace("\n", "\n  ")).Append("\n");
            sb.Append("  Proof: ").Append(Proof).Append("\n");
            sb.Append("}\n");
            return sb.ToString();
        }

        /// <summary>
        /// Returns the JSON string presentation of the object
        /// </summary>
        /// <returns>JSON string presentation of the object</returns>
        public override string ToJson()
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
            return this.Equals(input as PushToAuthZone);
        }

        /// <summary>
        /// Returns true if PushToAuthZone instances are equal
        /// </summary>
        /// <param name="input">Instance of PushToAuthZone to be compared</param>
        /// <returns>Boolean</returns>
        public bool Equals(PushToAuthZone input)
        {
            if (input == null)
                return false;

            return base.Equals(input) && 
                (
                    this.Proof == input.Proof ||
                    (this.Proof != null &&
                    this.Proof.Equals(input.Proof))
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
                int hashCode = base.GetHashCode();
                if (this.Proof != null)
                    hashCode = hashCode * 59 + this.Proof.GetHashCode();
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
            foreach(var x in base.BaseValidate(validationContext)) yield return x;
            yield break;
        }
    }

}
