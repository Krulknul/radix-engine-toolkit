/*
 * Transaction Library
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
using OpenAPIDateConverter = Org.OpenAPITools.Client.OpenAPIDateConverter;

namespace Org.OpenAPITools.Model
{
    /// <summary>
    /// An enum representing the type of the type of the entity being addressed
    /// </summary>
    /// <value>An enum representing the type of the type of the entity being addressed</value>
    
    [JsonConverter(typeof(StringEnumConverter))]
    
    public enum AddressKind
    {
        /// <summary>
        /// Enum Resource for value: Resource
        /// </summary>
        [EnumMember(Value = "Resource")]
        Resource = 1,

        /// <summary>
        /// Enum Package for value: Package
        /// </summary>
        [EnumMember(Value = "Package")]
        Package = 2,

        /// <summary>
        /// Enum AccountComponent for value: AccountComponent
        /// </summary>
        [EnumMember(Value = "AccountComponent")]
        AccountComponent = 3,

        /// <summary>
        /// Enum SystemComponent for value: SystemComponent
        /// </summary>
        [EnumMember(Value = "SystemComponent")]
        SystemComponent = 4,

        /// <summary>
        /// Enum NormalComponent for value: NormalComponent
        /// </summary>
        [EnumMember(Value = "NormalComponent")]
        NormalComponent = 5

    }

}